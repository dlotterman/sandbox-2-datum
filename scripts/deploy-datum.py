#!/usr/bin/env python3
"""Deploy (or remove) the inside agent, Postgres and rclone on Datum Compute, optionally under a name suffix.

The manifests in deploy/datum/ use fixed names (a2a-inside, a2a-postgres, ...). `--suffix b` renames every one of them
(a2a-inside-b, a2a-iroh-b, ...) as it applies them, so a second, independent stack can run beside the first without
touching it. Without a suffix the original names are used.

    scripts/deploy-datum.py up --suffix b --llm-env ../datum-loreate/.env --inside-key keys/inside-b.key \\
        --enrollment-token-file keys/enrollment-b.token --inside-digest sha256:...
    scripts/deploy-datum.py down --suffix b

`datumctl` runs from its container (override the command with DATUMCTL). Steps are spaced out and retried: Datum
occasionally answers "your request took too long to be checked against your quota", and succeeds a moment later.
"""
import argparse
import json
import os
import re
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATUM_DIR = os.path.join(ROOT, "deploy", "datum")
DEFAULT_DATUMCTL = (
    "podman run -i --rm --http-proxy=false --network host -v datumctl-config:/root/.datumctl:z "
    "docker.io/dlotterman/datumctl-container:latest"
)
# Every resource name in the manifests. `inside-env` comes before `inside` so the longer name wins.
NAMES = re.compile(r"\ba2a-(iroh|inside-env|inside|postgres|rclone|data-env|addrs)\b")


def rename(text, suffix):
    return NAMES.sub(lambda match: f"{match.group(0)}-{suffix}", text) if suffix else text


def datumctl(*args, stdin=None):
    command = os.environ.get("DATUMCTL", DEFAULT_DATUMCTL).split() + list(args)
    return subprocess.run(command, input=stdin, capture_output=True, text=True)


def apply(text, what):
    """`datumctl apply` with a few retries for Datum's occasional quota-check timeouts."""
    for attempt in range(1, 6):
        result = datumctl("apply", "-f", "-", stdin=text)
        output = (result.stdout + result.stderr).strip()
        if result.returncode == 0:
            print(f"  {what}: {output.splitlines()[-1] if output else 'ok'}")
            return
        if "quota" not in output.lower():
            sys.exit(f"{what} failed: {output[:400]}")
        print(f"  {what}: quota check timed out (attempt {attempt}), retrying in 30 s")
        time.sleep(30)
    sys.exit(f"{what}: still failing after retries")


def run_script(script, *args):
    """Run one of the secret-making scripts and return its manifest text."""
    result = subprocess.run([sys.executable, os.path.join(DATUM_DIR, script), *args], capture_output=True, text=True)
    if result.returncode != 0:
        sys.exit(f"{script} failed: {result.stderr.strip()[:300]}")
    return result.stdout


def manifest(filename, suffix, inside_digest=None):
    with open(os.path.join(DATUM_DIR, filename)) as handle:
        text = handle.read()
    if inside_digest:
        text = re.sub(r"(sandbox-2-datum@)sha256:[0-9a-f]{64}", rf"\1{inside_digest}", text)
    return rename(text, suffix)


def instance_address(workload):
    """The first address of the workload's /96 block (there is no private DNS yet), or None if it has no instance."""
    result = datumctl("compute", "instances", f"--workload={workload}", "--no-headers")
    for line in result.stdout.splitlines():
        columns = line.split()
        if len(columns) >= 4 and "/" in columns[3]:
            return columns[3].split("/")[0]
    return None


def wait_available(workloads, minutes=6):
    deadline = time.time() + minutes * 60
    while time.time() < deadline:
        listing = datumctl("compute", "workloads").stdout
        ready = [w for w in workloads if any(l.split()[0] == w and "Available" in l for l in listing.splitlines() if l.split())]
        if len(ready) == len(workloads):
            return
        time.sleep(10)
    sys.exit(f"timed out waiting for {workloads} to become Available")


def up(args):
    suffix = args.suffix
    names = {key: rename(f"a2a-{key}", suffix) for key in ("iroh", "inside", "postgres", "rclone", "data-env", "inside-env", "addrs")}
    print(f"deploying the stack as: {', '.join(names.values())}")

    apply(manifest("network.yaml", suffix), f"network {names['iroh']}")
    time.sleep(8)
    apply(rename(run_script("data-secret.py"), suffix), f"secret {names['data-env']}")
    inside_secret_args = [args.llm_env, args.inside_key, args.allowed_ids]
    if args.enrollment_token_file:
        inside_secret_args.append(args.enrollment_token_file)
    apply(rename(run_script("inside-secret.py", *inside_secret_args), suffix), f"secret {names['inside-env']}")
    time.sleep(5)
    apply(manifest("postgres.yaml", suffix), f"postgres {names['postgres']}")
    apply(manifest("rclone.yaml", suffix), f"rclone {names['rclone']}")
    print("  waiting for Postgres and rclone to become Available")
    wait_available([names["postgres"], names["rclone"]])
    time.sleep(20)

    addresses = {kind: instance_address(names[kind]) for kind in ("postgres", "rclone")}
    if not all(addresses.values()):
        sys.exit(f"could not read instance addresses: {addresses}")
    configmap = {
        "apiVersion": "v1", "kind": "ConfigMap", "metadata": {"name": names["addrs"]},
        "data": {"POSTGRES_HOST": addresses["postgres"], "RCLONE_HOST": addresses["rclone"]},
    }
    apply(json.dumps(configmap), f"configmap {names['addrs']}")
    time.sleep(6)
    apply(manifest("inside.yaml", suffix, args.inside_digest), f"inside agent {names['inside']}")
    print("  waiting for the inside agent")
    wait_available([names["inside"]])
    time.sleep(25)

    logs = datumctl("compute", "logs", names["inside"]).stdout
    found = re.findall(r"ENDPOINT_ID=([0-9a-f]{64})", logs)
    print(f"inside agent endpoint id: {found[-1] if found else '(not in the logs yet)'}")
    print("  " + "\n  ".join(line.split("] ", 1)[-1][:120] for line in logs.splitlines() if "allowlist" in line or "postgres ready" in line))


def down(args):
    suffix = args.suffix
    names = {key: rename(f"a2a-{key}", suffix) for key in ("iroh", "inside", "postgres", "rclone", "data-env", "inside-env", "addrs")}
    for workload in (names["inside"], names["postgres"], names["rclone"]):
        result = datumctl("compute", "destroy", workload)
        print(f"  destroy {workload}: {(result.stdout + result.stderr).strip().splitlines()[-1:]}")
        time.sleep(3)
    time.sleep(15)
    for service in (names["postgres"], names["rclone"]):
        datumctl("delete", "networkservice", service)
    for kind, name in (("secret", names["data-env"]), ("secret", names["inside-env"]), ("configmap", names["addrs"])):
        result = datumctl("delete", kind, name)
        print(f"  delete {kind} {name}: {(result.stdout + result.stderr).strip().splitlines()[-1:]}")
    result = datumctl("delete", "network", names["iroh"])
    print(f"  delete network {names['iroh']}: {(result.stdout + result.stderr).strip().splitlines()[-1:]}")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)

    create = commands.add_parser("up")
    create.add_argument("--suffix", default="", help="name suffix for every resource, e.g. b gives a2a-inside-b")
    create.add_argument("--llm-env", required=True, help=".env file holding BASETEN_API_KEY and BASETEN_MODEL")
    create.add_argument("--inside-key", required=True, help="file with the inside agent's secret key (hex): its stable identity")
    create.add_argument("--allowed-ids", default="", help="comma-separated endpoint ids always allowed (may be empty with enrollment)")
    create.add_argument("--enrollment-token-file", help="file with the shared enrollment token")
    create.add_argument("--inside-digest", help="sha256:... of the inside image to run (default: the digest in inside.yaml)")

    remove = commands.add_parser("down")
    remove.add_argument("--suffix", default="")

    args = parser.parse_args()
    {"up": up, "down": down}[args.command](args)


if __name__ == "__main__":
    main()
