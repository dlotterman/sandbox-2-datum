#!/usr/bin/env python3
"""Inventory the Rust crate graph and the built container images (with Syft); write sbom.md.

Needs rootless Podman, a prior `scripts/build.sh` (binaries live in the `a2a-target` volume), and network access
(base images, Syft image, crate sources if not cached). Run from anywhere:

    scripts/generate-sbom.py [--output sbom.md]

Everything runs in disposable containers; nothing is installed on the host.
"""

import argparse
import collections
import datetime
import hashlib
import json
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUST_IMAGE = "docker.io/library/rust:1-bookworm"
SYFT_IMAGE = "docker.io/anchore/syft:latest"
BINARIES = ("inside-agent", "outside-agent")
IMAGES = {  # name -> (Containerfile, build context)
    "inside-agent": ("deploy/Containerfile.inside", "bin"),
    "outside-agent": ("deploy/Containerfile.outside", "bin"),
    "rclone": ("deploy/Containerfile.rclone", "."),
}
# Hand-written context for crates that need more than an SPDX string. Keep entries short and factual.
NOTES = {
    "ring": "Declared Apache-2.0 AND ISC; the source also ships LICENSE-BoringSSL and LICENSE-other-bits, so it carries BoringSSL-derived code and third-party notices that must travel with redistributions.",
    "matchit": "MIT AND BSD-3-Clause; includes code derived from httprouter (LICENSE.httprouter).",
    "attohttpc": "MPL-2.0 is file-level copyleft: changes to its own files must be shared; this project uses it unmodified.",
    "webpki-roots": "Mozilla CA root certificate data, licensed CDLA-Permissive-2.0.",
    "blake3": "Triple choice: CC0-1.0, Apache-2.0, or Apache-2.0 with the LLVM exception.",
    "unicode-ident": "Unicode data under Unicode-3.0 in addition to MIT OR Apache-2.0.",
    "unicode-normalization": "Unicode data under Unicode-3.0 in addition to MIT OR Apache-2.0.",
}
COMMON = {"MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unlicense", "0BSD", "Zlib", "CC0-1.0", "BSL-1.0"}

# Runs inside the Rust container. Prints one JSON document describing the crates linked into each binary.
CARGO_SCRIPT = r'''
import json, os, re, subprocess
def run(*a):
    return subprocess.check_output(a, text=True, cwd="/work")
md = json.loads(run("cargo", "metadata", "--format-version", "1", "--locked"))
pk = {p["id"]: p for p in md["packages"]}
workspace = sorted(pk[i]["name"] for i in md["workspace_members"])
direct = {d["name"] for i in md["workspace_members"] for d in pk[i]["dependencies"] if d["kind"] is None}
by_key = {(p["name"], p["version"]): p for p in md["packages"]}
linked = {}
for binary in %(binaries)r:
    for line in run("cargo", "tree", "--locked", "-p", binary, "-e", "normal", "--prefix", "none", "--format", "{p}").splitlines():
        m = re.match(r"(\S+) v(\S+)(.*)$", line)
        if not m or m.group(1) in workspace:
            continue
        key = (m.group(1), m.group(2))
        entry = linked.setdefault(key, {"binaries": set(), "proc_macro": "(proc-macro)" in m.group(3)})
        entry["binaries"].add(binary)
out = []
for (name, version), entry in sorted(linked.items()):
    p = by_key[(name, version)]
    d = os.path.dirname(p["manifest_path"])
    files = sorted(f for f in os.listdir(d) if re.match(r"(?i)(licen[cs]e|copying|notice|unlicense|copyright)", f))
    out.append({"name": name, "version": version, "license": p.get("license"), "license_file": p.get("license_file"),
                "repository": p.get("repository"), "direct": name in direct, "proc_macro": entry["proc_macro"],
                "binaries": sorted(entry["binaries"]), "evidence": files})
print(json.dumps({"rustc": run("rustc", "--version").strip(), "cargo": run("cargo", "--version").strip(),
                  "workspace": workspace, "crates": out}))
''' % {"binaries": BINARIES}


def run(*args, **kw):
    return subprocess.check_output(args, text=True, **kw)


def cell(value):
    return str(value).replace("|", "&#124;").replace("\n", " ")


def spdx_ids(expr):
    return {p for p in re.split(r"\s+(?:AND|OR|WITH)\s+|[()/]|\s+", expr or "") if p and p not in ("AND", "OR", "WITH")}


def cargo_inventory():
    out = run("podman", "run", "--rm", "--http-proxy=false", "-v", "cargo-home:/usr/local/cargo/registry:z",
              "-v", f"{ROOT}:/work:ro,z", "-w", "/work", RUST_IMAGE, "python3", "-c", CARGO_SCRIPT)
    return json.loads(out)


def scan_images(tmp):
    """Build each image from its Containerfile, export it, and scan it with Syft."""
    bin_dir = Path(tmp) / "bin"
    bin_dir.mkdir()
    run("podman", "run", "--rm", "--http-proxy=false", "-v", "a2a-target:/t:ro,z", "-v", f"{bin_dir}:/out:z",
        "docker.io/library/debian:bookworm-slim", "sh", "-c", "cp /t/release/inside-agent /t/release/outside-agent /out/")
    results = {}
    for name, (containerfile, ctx) in IMAGES.items():
        tag = f"localhost/a2a-sbom-{name}:sbom"
        context = str(bin_dir) if ctx == "bin" else str(ROOT)
        run("podman", "build", "-q", "--http-proxy=false", "-t", tag, "-f", str(ROOT / containerfile), context)
        image_id = run("podman", "image", "inspect", tag, "--format", "{{.Id}}").strip()
        archive = Path(tmp) / f"{name}.oci"
        run("podman", "save", "--format", "oci-archive", "-o", str(archive), tag)
        syft_json = Path(tmp) / f"{name}.syft.json"
        run("podman", "run", "--rm", "--http-proxy=false", "-v", f"{tmp}:/in:z", SYFT_IMAGE,
            f"oci-archive:/in/{archive.name}", "-o", f"syft-json=/in/{syft_json.name}", "-q")
        data = json.loads(syft_json.read_text())
        results[name] = {
            "image_id": image_id,
            "distro": (data.get("distro") or {}).get("prettyName", "unknown"),
            "syft": data["descriptor"]["version"],
            "packages": sorted(
                ({"name": a["name"], "version": a.get("version", ""), "type": a["type"],
                  "licenses": sorted({(l.get("spdxExpression") or l["value"]) for l in a.get("licenses", [])
                                      if not l.get("value", "").startswith("sha256:")})}
                 for a in data["artifacts"]),
                key=lambda p: (p["type"], p["name"])),
        }
    return results


def md_table(header, rows):
    lines = ["| " + " | ".join(header) + " |", "| " + " | ".join("---" for _ in header) + " |"]
    lines += ["| " + " | ".join(cell(c) for c in r) + " |" for r in rows]
    return "\n".join(lines)


def render(cargo, images):
    crates = cargo["crates"]
    lock_sha = hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest()
    today = datetime.date.today().isoformat()
    syft_version = next(iter(images.values()))["syft"]

    counts = collections.Counter()
    for c in crates:
        for i in spdx_ids(c["license"]):
            counts[i] += 1
    unusual = sorted((c for c in crates if spdx_ids(c["license"]) - COMMON or not c["license"] or " AND " in (c["license"] or "") or c["name"] in NOTES),
                     key=lambda c: c["name"])

    out = [f"# Software bill of materials\n",
           f"Generated {today} with Syft {syft_version}, {cargo['rustc']} and {cargo['cargo']}. Regenerate with "
           "[`scripts/generate-sbom.py`](scripts/generate-sbom.py).\n",
           "## Scope and provenance\n",
           f"This snapshot covers the Rust crates linked into the two executables (`{'`, `'.join(BINARIES)}`, "
           "resolved by `cargo tree -e normal` from the committed lockfile) and the packages Syft detected in the three "
           "container images built from this repository's Containerfiles. Versions are the locked or observed versions.\n",
           f"- Crates linked into the executables: **{len(crates)}** "
           f"({sum(1 for c in crates if c['direct'])} direct, {sum(1 for c in crates if c['proc_macro'])} are "
           "procedural macros that run at build time only).",
           f"- `Cargo.lock` SHA256: `{lock_sha}`.",
           "- Workspace crates (this project, The Unlicense): " + ", ".join(f"`{w}`" for w in cargo["workspace"]) + ".",
           "- Build-only and test-only crates (`-e normal` excludes them) and the Rust toolchain are not part of the "
           "executables and are not listed.",
           "- The Containerfiles use mutable base tags (`debian:bookworm-slim`, `rclone/rclone:latest`), so rebuilding can "
           "change the operating-system package set. The image IDs below identify what was scanned.\n",
           "## License interpretation\n",
           "The Unlicense covers only this project's original code, documentation and packaging. Third-party software keeps "
           "its own licenses and notices. License identifiers below describe the component they are listed against; they "
           "are not a choice of license for the whole application.\n",
           "Crate licenses are the SPDX expressions declared in each crate's `Cargo.toml` (for example `MIT OR Apache-2.0` "
           "means either license may be selected). The *Evidence* column lists license and notice files found in the "
           "crate's published source. This is package-level metadata, not a file-level audit of the source. Container "
           "license expressions are Syft's observations of package metadata and installed copyright files; "
           "`NOASSERTION` means it could not establish a license, not that the component is unlicensed.\n"]

    out += ["## Crate license summary\n",
            "Counts of linked crates mentioning each identifier (an `OR` expression counts once for each alternative).\n",
            md_table(["License identifier", "Crates"], [(k, v) for k, v in counts.most_common()]), ""]

    out += ["## Worth calling out\n",
            "Crates whose license expression includes something other than the common permissive set "
            f"({', '.join(sorted(COMMON))}), declares no SPDX expression, or combines licenses with `AND`:\n"]
    if unusual:
        out.append(md_table(["Crate", "Version", "License", "Evidence", "Note"],
                            [(c["name"], c["version"], c["license"] or f"none declared (license_file: {c['license_file']})",
                              "; ".join(c["evidence"]) or "none found",
                              NOTES.get(c["name"], "Unicode data license" if "Unicode-3.0" in (c["license"] or "") else ""))
                             for c in unusual]))
    else:
        out.append("None.")
    copyleft = []
    for name, img in images.items():
        flagged = collections.Counter()
        for p in img["packages"]:
            for l in p["licenses"]:
                if re.search(r"GPL|MPL|Artistic|EPL|CDDL", l):
                    flagged[l] += 1
        if flagged:
            copyleft.append((name, ", ".join(f"{k} ({v})" for k, v in flagged.most_common(8))))
    out.append("")
    def short(names, limit=12):
        return ", ".join(names[:limit]) + (f" (+{len(names) - limit} more, see the package list below)" if len(names) > limit else "")
    unknown = [(n, [p["name"] for p in i["packages"] if not p["licenses"] or "NOASSERTION" in p["licenses"]])
               for n, i in images.items()]
    unknown = [(n, short(pk)) for n, pk in unknown if pk]
    if unknown:
        out.append("Operating-system packages for which Syft found no license (it cannot tell us they are unlicensed; "
                   "for example glibc is LGPL-2.1-or-later with exceptions):\n")
        out.append(md_table(["Image", "Packages with NOASSERTION"], unknown))
        out.append("")
    if copyleft:
        out.append("Copyleft and weak-copyleft licenses appear on operating-system packages inside the images (not in the "
                   "Rust executables). They apply to those packages, which are distributed in the images unmodified:\n")
        out.append(md_table(["Image", "License identifiers on OS packages (package counts)"], copyleft))
        out.append("")
    out += ["## Redistribution notes\n",
            "- Most linked crates are MIT, Apache-2.0 or BSD-style. Their terms require keeping copyright and license "
            "notices when you redistribute the binaries or images. The published images do not currently bundle those notice "
            "texts; the *Evidence* column shows where they live in each crate's source. Add them to the image or distribute "
            "them alongside it if you publish builds.",
            "- The Mozilla CA bundle (`webpki-roots` or the OS `ca-certificates` package) carries its own license terms "
            "(see the callout table and the image inventory).",
            "- This project does not bundle the Datum fonts or artwork; the dashboard only borrows a color palette and type "
            "feel from datum.net and falls back to system fonts. Datum's name and branding are not licensed by The Unlicense.",
            "- Runtime services used over the network (the LLM API, an IP-geolocation lookup, Datum's Iroh relays, and the "
            "hosting platforms) have their own terms and are not software redistributed here.",
            "- Postgres (`docker.io/library/postgres`) is pulled unmodified at deploy time under the PostgreSQL License "
            "plus its Debian packages' licenses; it is not built or redistributed by this repository. The rclone image "
            "extends the upstream `rclone/rclone` image (MIT) with the demo files in `demo/rclone-data`.\n"]

    out += ["## Rust crates linked into the executables\n",
            md_table(["Crate", "Version", "Relationship", "Used by", "License", "Evidence"],
                     [(f"[{c['name']}](https://crates.io/crates/{c['name']}/{c['version']})", c["version"],
                       ("Direct" if c["direct"] else "Transitive") + (" (proc-macro)" if c["proc_macro"] else ""),
                       ", ".join(c["binaries"]), c["license"] or "see license_file", "; ".join(c["evidence"]) or "none found")
                      for c in crates]), ""]

    out += ["## Container images\n",
            "Each image is a Debian or Alpine base plus one Rust executable (dynamically linked against glibc) or the rclone binary. "
            "Executables built by Rust contain the crates above; Syft cannot list them from a stripped binary.\n"]
    for name, img in images.items():
        by_type = collections.Counter(p["type"] for p in img["packages"])
        lic = collections.Counter(l for p in img["packages"] for l in p["licenses"])
        out += [f"### {name}\n",
                f"Image ID `{img['image_id']}`; base `{img['distro']}`; **{len(img['packages'])}** packages detected "
                f"({', '.join(f'{v} {k}' for k, v in by_type.most_common())}).\n",
                "<details>\n<summary>Package list</summary>\n",
                md_table(["Package", "Version", "Type", "License identifiers found"],
                         [(p["name"], p["version"], p["type"], "; ".join(p["licenses"]) or "NOASSERTION") for p in img["packages"]]),
                "\n</details>\n"]

    out += ["## Refresh\n",
            "Needs rootless Podman, network access, and a prior `scripts/build.sh` (it leaves the binaries in the "
            "`a2a-target` volume). Everything else runs in disposable containers:\n",
            "```sh\nscripts/build.sh\nscripts/generate-sbom.py\n```\n",
            "Review the diff of `sbom.md` before committing; changed licenses deserve a look."]
    return "\n".join(out) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--output", default=str(ROOT / "sbom.md"))
    args = ap.parse_args()
    cargo = cargo_inventory()
    with tempfile.TemporaryDirectory() as tmp:
        images = scan_images(tmp)
    Path(args.output).write_text(render(cargo, images))
    print(f"wrote {args.output}: {len(cargo['crates'])} crates, "
          + ", ".join(f"{n}: {len(i['packages'])} pkgs" for n, i in images.items()))


if __name__ == "__main__":
    main()
