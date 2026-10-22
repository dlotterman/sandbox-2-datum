#!/usr/bin/env python3
"""Run the outside agent on a Fly Sprite (not a Fly App: see scripts/deploy-fly.sh for that).

A Sprite is a persistent Linux VM that sleeps when idle, with no container runtime. So instead of running our image
this *unpacks* it: it copies in deploy/vm/unpack-image.py and install-outside-agent.sh, runs the installer (which
pulls the agent binary out of the public image), and defines the agent as a Sprite service, which Sprites starts at
boot, restarts on a crash, and brings back after a cold wake.

    scripts/deploy-sprite.py up --image ghcr.io/OWNER/REPO@sha256:... --inside-id <id or file> \\
        --enrollment-token-file keys/enrollment.token [--keep-awake]
    scripts/deploy-sprite.py down

Needs SPRITE_TOKEN in .env (create one at sprites.dev/account). The Fly API token used for Fly Apps does not work here.

A Sprite pauses about 30 seconds after it goes idle, freezing the agent and dropping its connections, so by default
the agent only shows as live in the registry while the Sprite is awake. `--keep-awake` adds a second service that
holds the Sprite open with the documented Tasks API heartbeat; the Sprite then bills continuously.
"""
import argparse
import json
import urllib.error
import urllib.parse
import urllib.request

import _vm_common as common

API = "https://api.sprites.dev/v1"
SERVICE = "a2a-outside"
KEEP_AWAKE_SERVICE = "a2a-keep-awake"
INSTALL_DIR = "/tmp/a2a-install"  # on the Sprite's persistent disk
DASHBOARD_PORT = 8080  # the Sprite's URL routes here by default

# The documented heartbeat: renew a 5-minute hold on the Sprite every minute, through the in-Sprite management socket.
KEEP_AWAKE_LOOP = (
    "while true; do "
    "sprite-env curl -X PUT http://sprite/v1/tasks/a2a-agent -d '{\"expire\": \"5m\"}' >/dev/null 2>&1; "
    "sleep 60; done"
)


class SpriteError(Exception):
    pass


class Sprites:
    def __init__(self, token):
        self.token = token

    def call(self, method, path, query=None, body=None, content_type="application/json", timeout=120):
        """One API call. A dict body is sent as JSON, bytes as-is. Returns (status, response text)."""
        url = f"{API}{path}" + (f"?{urllib.parse.urlencode(query)}" if query else "")
        data = json.dumps(body).encode() if isinstance(body, (dict, list)) else body
        request = urllib.request.Request(
            url,
            method=method,
            data=data,
            headers={"Authorization": f"Bearer {self.token}", "Content-Type": content_type, "User-Agent": "sandbox-2-datum"},
        )
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                return response.status, response.read().decode(errors="replace")
        except urllib.error.HTTPError as error:
            return error.code, error.read().decode(errors="replace")

    def ok(self, method, path, **kwargs):
        status, text = self.call(method, path, **kwargs)
        if status >= 300:
            raise SpriteError(f"{method} {path}: HTTP {status} {text[:300]}")
        return text


def token_from_env():
    return common.load_dotenv().get("SPRITE_TOKEN") or common.fail("SPRITE_TOKEN is not set in .env")


def up(args):
    settings = common.install_environment(args, http_bind=f"0.0.0.0:{DASHBOARD_PORT}")
    if args.public_url and "DASHBOARD_TOKEN" not in settings:
        common.fail("a public Sprite URL needs --dashboard-token-file: the dashboard shows runtime facts")
    sprites = Sprites(token_from_env())
    name = args.name

    status, text = sprites.call("POST", "/sprites", body={"name": name, "url_settings": {"auth": "public" if args.public_url else "sprite"}})
    if status == 201:
        print(f"created sprite {name}")
    elif status in (400, 409) and "exist" in text.lower():
        print(f"sprite {name} already exists; reusing it")
    else:
        raise SpriteError(f"create sprite: HTTP {status} {text[:300]}")

    # The two helper files go in as raw bytes; the installer is made executable.
    for filename, mode in (("unpack-image.py", "0755"), ("install-outside-agent.sh", "0755")):
        sprites.ok(
            "PUT", f"/sprites/{name}/fs/write",
            query={"path": f"{INSTALL_DIR}/{filename}", "workingDir": "/", "mode": mode, "mkdir": "true"},
            body=common.vm_file(filename), content_type="application/octet-stream",
        )
    print("copied the unpacker and installer onto the sprite")

    # Run the installer. Settings travel as environment variables of this one command, never as files we leave behind.
    command = [("cmd", "sh"), ("cmd", f"{INSTALL_DIR}/install-outside-agent.sh")]
    environment = [("env", f"{key}={value}") for key, value in settings.items()]
    output = sprites.ok("POST", f"/sprites/{name}/exec", query=command + environment, timeout=300)
    print("installer output (tail):\n  " + "\n  ".join(output.strip().splitlines()[-6:]))

    sprites.ok("PUT", f"/sprites/{name}/services/{SERVICE}",
               body={"cmd": "/usr/local/bin/a2a-outside-run", "args": [], "http_port": DASHBOARD_PORT})
    print(f"service {SERVICE} created (starts at boot, restarts on crash, wakes the sprite on a dashboard request)")

    if args.keep_awake:
        sprites.ok("PUT", f"/sprites/{name}/services/{KEEP_AWAKE_SERVICE}", body={"cmd": "sh", "args": ["-c", KEEP_AWAKE_LOOP]})
        print(f"service {KEEP_AWAKE_SERVICE} created: the sprite is held awake and bills continuously")

    details = json.loads(sprites.ok("GET", f"/sprites/{name}"))
    print(f"up: {details.get('url', '(see sprites.dev)')}   (url auth: {'public' if args.public_url else 'org members only'})")
    print("  the agent should appear in any dashboard's registry table within a couple of minutes")


def down(args):
    status, text = Sprites(token_from_env()).call("DELETE", f"/sprites/{args.name}")
    if status in (200, 204):
        print(f"deleted sprite {args.name}")
    elif status == 404:
        print(f"no sprite named {args.name}")
    else:
        raise SpriteError(f"delete sprite: HTTP {status} {text[:300]}")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)

    create = commands.add_parser("up", help="create the sprite and install the agent")
    common.add_install_arguments(create)
    create.add_argument("--name", default="a2a-outside")
    create.add_argument("--keep-awake", action="store_true", help="hold the sprite awake so the agent stays live (bills continuously)")
    create.add_argument("--public-url", action="store_true", help="make the sprite's URL public (requires --dashboard-token-file)")

    remove = commands.add_parser("down", help="delete the sprite")
    remove.add_argument("--name", default="a2a-outside")

    try:
        common.main_with(parser, {"up": up, "down": down})
    except SpriteError as error:
        common.fail(str(error))


if __name__ == "__main__":
    main()
