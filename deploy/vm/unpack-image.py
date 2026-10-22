#!/usr/bin/env python3
"""Pull one file out of a public container image, with no container runtime.

VM-like targets (Vultr, Fly Sprites, any plain Linux box) cannot run our container image the way a container
platform does, but the agent is a single binary sitting in the image's last layer. This fetches exactly that layer
from the registry and unpacks the one file.

    unpack-image.py ghcr.io/OWNER/REPO@sha256:<digest> usr/local/bin/outside-agent /usr/local/bin/outside-agent

Only the Python standard library is used. The image must be public (anonymous pulls). A reference pinned by digest
is verified: the manifest must hash to that digest, and every layer must hash to the digest the manifest lists, so
what lands on disk is exactly what was published. The layers are searched from the top down and the search stops at
the first one containing the file, which for our images means downloading about 8 MB instead of 40.
"""
import hashlib
import json
import os
import platform
import re
import sys
import tarfile
import tempfile
import urllib.error
import urllib.request

ACCEPT = ", ".join(
    [
        "application/vnd.oci.image.index.v1+json",
        "application/vnd.docker.distribution.manifest.list.v2+json",
        "application/vnd.oci.image.manifest.v1+json",
        "application/vnd.docker.distribution.manifest.v2+json",
    ]
)
INDEX_TYPES = ("image.index", "manifest.list")
ARCHITECTURES = {"x86_64": "amd64", "amd64": "amd64", "aarch64": "arm64", "arm64": "arm64"}


def parse_reference(reference):
    """`ghcr.io/owner/repo@sha256:...` or `ghcr.io/owner/repo:tag` -> (registry, repository, tag-or-digest)."""
    match = re.fullmatch(r"([^/]+)/(.+?)(?:@(sha256:[0-9a-f]{64})|:([\w.-]+))?", reference)
    if not match:
        sys.exit(f"cannot parse image reference {reference!r}")
    registry, repository, digest, tag = match.groups()
    return registry, repository, digest or tag or "latest"


class Registry:
    """Anonymous, read-only access to one repository."""

    def __init__(self, registry, repository):
        self.base = f"https://{registry}/v2/{repository}"
        self.repository = repository
        self.token = None

    def get(self, path, accept=None):
        """GET a registry path; authenticates anonymously on the first 401 and retries once."""
        for attempt in (1, 2):
            request = urllib.request.Request(self.base + path, headers={"User-Agent": "a2a-unpack/1"})
            if accept:
                request.add_header("Accept", accept)
            if self.token:
                request.add_header("Authorization", f"Bearer {self.token}")
            try:
                return urllib.request.urlopen(request, timeout=60)
            except urllib.error.HTTPError as error:
                if error.code != 401 or attempt == 2:
                    sys.exit(f"registry returned HTTP {error.code} for {path}")
                self.token = self.anonymous_token(error.headers.get("WWW-Authenticate", ""))

    def anonymous_token(self, challenge):
        """Answer a `WWW-Authenticate: Bearer realm=...,service=...` challenge with an anonymous pull token."""
        fields = dict(re.findall(r'(\w+)="([^"]*)"', challenge))
        if "realm" not in fields:
            sys.exit("registry wants credentials; this tool only pulls public images")
        scope = fields.get("scope") or f"repository:{self.repository}:pull"
        url = f"{fields['realm']}?service={fields.get('service', '')}&scope={scope}"
        with urllib.request.urlopen(url, timeout=60) as response:
            return json.load(response)["token"]


def fetch_manifest(registry, reference):
    """The image manifest for this machine's architecture, following an index if needed. Digest pins are verified."""
    with registry.get(f"/manifests/{reference}", accept=ACCEPT) as response:
        body = response.read()
        media_type = response.headers.get("Content-Type", "")
    if reference.startswith("sha256:") and "sha256:" + hashlib.sha256(body).hexdigest() != reference:
        sys.exit(f"manifest does not hash to {reference}")
    manifest = json.loads(body)
    if not any(kind in media_type or kind in manifest.get("mediaType", "") for kind in INDEX_TYPES):
        return manifest

    architecture = ARCHITECTURES.get(platform.machine(), platform.machine())
    for entry in manifest["manifests"]:
        target = entry.get("platform", {})
        if target.get("os") == "linux" and target.get("architecture") == architecture:
            return fetch_manifest(registry, entry["digest"])
    sys.exit(f"this image has no linux/{architecture} build")


def find_in_layer(registry, layer, wanted, destination):
    """Download one layer (verifying its digest) and, if it contains `wanted`, write it to `destination`."""
    expected = layer["digest"]
    if "zstd" in layer.get("mediaType", ""):
        sys.exit("this image uses zstd layers, which the Python standard library cannot read")

    with tempfile.NamedTemporaryFile() as blob, registry.get(f"/blobs/{expected}") as response:
        digest = hashlib.sha256()
        while chunk := response.read(1 << 20):
            digest.update(chunk)
            blob.write(chunk)
        if "sha256:" + digest.hexdigest() != expected:
            sys.exit(f"layer {expected} was corrupted in transit")
        blob.flush()

        with tarfile.open(blob.name, mode="r:*") as archive:
            for member in archive:
                if member.isfile() and member.name.lstrip("./") == wanted:
                    install(archive.extractfile(member), destination, member.mode)
                    return True
    return False


def install(source, destination, mode):
    """Write atomically, so a half-written binary is never left where a service might start it."""
    os.makedirs(os.path.dirname(destination) or ".", exist_ok=True)
    temporary = destination + ".partial"
    with open(temporary, "wb") as out:
        while chunk := source.read(1 << 20):
            out.write(chunk)
    os.chmod(temporary, mode & 0o777)
    os.replace(temporary, destination)


def main():
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    reference, wanted, destination = sys.argv[1], sys.argv[2].lstrip("/"), sys.argv[3]
    host, repository, pinned = parse_reference(reference)
    registry = Registry(host, repository)
    manifest = fetch_manifest(registry, pinned)

    for layer in reversed(manifest["layers"]):
        if find_in_layer(registry, layer, wanted, destination):
            print(f"unpacked {wanted} from {reference} to {destination}")
            return
    sys.exit(f"{wanted} not found in any layer of {reference}")


if __name__ == "__main__":
    main()
