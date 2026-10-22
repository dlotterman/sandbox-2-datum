#!/usr/bin/env python3
"""Run the outside agent on a Vultr cloud instance.

A Vultr instance is a plain VM with no container runtime, so instead of running our image it *unpacks* it: the
instance's first-boot script (cloud-init user data) fetches the agent binary out of the public image
(deploy/vm/unpack-image.py), writes its settings, and starts it under systemd (deploy/vm/install-outside-agent.sh).

    scripts/deploy-vultr.py up --image ghcr.io/OWNER/REPO@sha256:... --inside-id <id or file> \\
        --enrollment-token-file keys/enrollment.token
    scripts/deploy-vultr.py down

Needs VULTR (an API key) in .env. Vultr API keys have an IP access list; add this machine under Account > API.
The instance gets a throwaway SSH key (keys/vultr_ed25519) and password logins are turned off. The dashboard listens
on localhost only; reach it with `ssh -L 8080:localhost:8080 -i keys/vultr_ed25519 root@IP`. The agent shows up in
every other agent's registry table regardless, which is the point.
"""
import argparse
import base64
import json
import os
import shlex
import subprocess
import sys
import time
import urllib.error
import urllib.request

import _vm_common as common

API = "https://api.vultr.com/v2"
DEBIAN_12 = 2136  # Vultr's os_id for "Debian 12 x64 (bookworm)"
KEY_PATH = os.path.join(common.ROOT, "keys", "vultr_ed25519")
HOW_LONG_TO_WAIT_FOR_ACTIVE = 600  # seconds


class VultrError(Exception):
    pass


def api(token, method, path, body=None):
    """One Vultr API call. Returns the decoded JSON body, or {} for an empty one (deletes)."""
    request = urllib.request.Request(
        API + path,
        method=method,
        data=json.dumps(body).encode() if body is not None else None,
        headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json", "User-Agent": "sandbox-2-datum"},
    )
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            payload = response.read()
            return json.loads(payload) if payload else {}
    except urllib.error.HTTPError as error:
        detail = error.read().decode()
        if error.code == 401 and "IP" in detail:
            raise VultrError("Vultr rejected this machine's IP address. Add it under Account > API > Access Control.")
        raise VultrError(f"{method} {path}: HTTP {error.code} {detail[:300]}")


def user_data(settings):
    """The first-boot script: write the two helper files, install the agent, and lock SSH down to keys."""
    exports = "\n".join(f"export {name}={shlex.quote(value)}" for name, value in settings.items())
    unpacker = common.vm_file("unpack-image.py").decode()
    installer = common.vm_file("install-outside-agent.sh").decode()
    return f"""#!/bin/sh
set -eu
mkdir -p /opt/a2a
cat > /opt/a2a/unpack-image.py <<'A2A_UNPACK_EOF'
{unpacker}
A2A_UNPACK_EOF
cat > /opt/a2a/install-outside-agent.sh <<'A2A_INSTALL_EOF'
{installer}
A2A_INSTALL_EOF
{exports}
sh /opt/a2a/install-outside-agent.sh

# Keys only: Vultr sets a root password by default, and nothing here needs it.
mkdir -p /etc/ssh/sshd_config.d
printf 'PasswordAuthentication no\\n' > /etc/ssh/sshd_config.d/00-a2a.conf
systemctl reload ssh || systemctl reload sshd || true
"""


def ensure_ssh_key(token, label):
    """Make (once) a throwaway keypair for this deployment and register its public half with Vultr."""
    if not os.path.exists(KEY_PATH):
        os.makedirs(os.path.dirname(KEY_PATH), exist_ok=True)
        subprocess.run(["ssh-keygen", "-q", "-t", "ed25519", "-N", "", "-C", label, "-f", KEY_PATH], check=True)
    public_key = common.read_text(KEY_PATH + ".pub")
    for existing in api(token, "GET", "/ssh-keys?per_page=100").get("ssh_keys", []):
        if existing["name"] == label:
            return existing["id"]
    return api(token, "POST", "/ssh-keys", {"name": label, "ssh_key": public_key})["ssh_key"]["id"]


def up(args):
    settings = common.install_environment(args, http_bind="127.0.0.1:8080")
    data = user_data(settings)

    if args.user_data_out:
        with open(os.open(args.user_data_out, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600), "w") as out:
            out.write(data)
    if args.dry_run:
        print(f"dry run: would create '{args.label}' ({args.plan} in {args.region}, Debian 12), user data {len(data)} bytes")
        return

    token = common.load_dotenv().get("VULTR") or common.fail("VULTR is not set in .env")
    existing = [i for i in api(token, "GET", "/instances?per_page=100").get("instances", []) if i["label"] == args.label]
    if existing:
        common.fail(f"an instance labelled '{args.label}' already exists ({existing[0]['id']}); run 'down' first so only one runs")
    key_id = ensure_ssh_key(token, args.label)
    created = api(token, "POST", "/instances", {
        "region": args.region,
        "plan": args.plan,
        "os_id": DEBIAN_12,
        "label": args.label,
        "hostname": args.label,
        "sshkey_id": [key_id],
        "user_data": base64.b64encode(data.encode()).decode(),
        "backups": "disabled",
        "enable_ipv6": False,
    })["instance"]
    print(f"created instance {created['id']}; waiting for it to come up (the agent installs on first boot)...")

    deadline = time.time() + HOW_LONG_TO_WAIT_FOR_ACTIVE
    while time.time() < deadline:
        instance = api(token, "GET", f"/instances/{created['id']}")["instance"]
        if instance["status"] == "active" and instance["main_ip"] not in ("", "0.0.0.0"):
            print(f"up: {instance['main_ip']}  ({instance['plan']}, {instance['region']})")
            print(f"  look in:  ssh -i {os.path.relpath(KEY_PATH)} root@{instance['main_ip']}  then  journalctl -u a2a-outside")
            print("  the agent should appear in any dashboard's registry table within a couple of minutes")
            return
        time.sleep(10)
    common.fail("the instance did not become active in time; check the Vultr dashboard")


def down(args):
    token = common.load_dotenv().get("VULTR") or common.fail("VULTR is not set in .env")
    for instance in api(token, "GET", "/instances?per_page=100").get("instances", []):
        if instance["label"] == args.label:
            api(token, "DELETE", f"/instances/{instance['id']}")
            print(f"deleted instance {instance['id']} ({instance['label']})")
    for key in api(token, "GET", "/ssh-keys?per_page=100").get("ssh_keys", []):
        if key["name"] == args.label:
            api(token, "DELETE", f"/ssh-keys/{key['id']}")
            print(f"deleted ssh key {key['name']}")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)

    create = commands.add_parser("up", help="create the instance")
    common.add_install_arguments(create)
    create.add_argument("--region", default="ewr", help="Vultr region id (default ewr; the cheapest plan is only offered there)")
    create.add_argument("--plan", default="vc2-1c-0.5gb", help="plan id (default vc2-1c-0.5gb, about $3.50/month; vc2-1c-1gb is $5 in most regions)")
    create.add_argument("--label", default="a2a-outside")
    create.add_argument("--dry-run", action="store_true", help="build everything but make no API calls")
    create.add_argument("--user-data-out", help="also write the first-boot script here (it contains secrets; mode 0600)")

    remove = commands.add_parser("down", help="delete the instance and its SSH key")
    remove.add_argument("--label", default="a2a-outside")

    try:
        common.main_with(parser, {"up": up, "down": down})
    except VultrError as error:
        common.fail(str(error))


if __name__ == "__main__":
    main()
