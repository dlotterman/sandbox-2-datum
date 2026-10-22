"""Shared pieces of the VM-style deploy scripts (Vultr, Fly Sprites): read credentials and build the install settings."""
import argparse
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VM_DIR = os.path.join(ROOT, "deploy", "vm")


def load_dotenv(path=None):
    """Parse `.env` by hand (never `source` it: values can contain spaces). Real environment variables win."""
    values = {}
    try:
        with open(path or os.path.join(ROOT, ".env")) as handle:
            for line in handle:
                line = line.strip()
                if line and not line.startswith("#") and "=" in line:
                    name, value = line.split("=", 1)
                    values[name.strip()] = value.strip().strip("\"'")
    except FileNotFoundError:
        pass
    values.update({name: os.environ[name] for name in values if name in os.environ})
    return values


def read_text(path):
    try:
        with open(path) as handle:
            return handle.read().strip()
    except OSError as error:
        fail(f"cannot read {path}: {error.strerror}")


def add_install_arguments(parser):
    """The settings every VM-style target needs to install the agent."""
    parser.add_argument("--image", required=True, help="public image pinned by digest: ghcr.io/OWNER/REPO@sha256:...")
    parser.add_argument("--inside-id", required=True, help="the inside agent's endpoint id, or a file containing it")
    access = parser.add_mutually_exclusive_group(required=True)
    access.add_argument("--enrollment-token-file", help="file with the inside agent's enrollment token (no key needed)")
    access.add_argument("--key-file", help="file with a pre-registered secret key (hex); its id must be allowlisted")
    parser.add_argument("--dashboard-token-file", help="optional file with a token to protect the dashboard")


def install_environment(args, http_bind):
    """The environment variables install-outside-agent.sh reads."""
    inside_id = read_text(args.inside_id) if os.path.exists(args.inside_id) else args.inside_id.strip()
    settings = {"A2A_IMAGE": args.image, "INSIDE_ENDPOINT_ID": inside_id, "HTTP_BIND": http_bind}
    if args.enrollment_token_file:
        settings["ENROLLMENT_TOKEN"] = read_text(args.enrollment_token_file)
    else:
        settings["OUTSIDE_SECRET_KEY_HEX"] = read_text(args.key_file)
    if args.dashboard_token_file:
        settings["DASHBOARD_TOKEN"] = read_text(args.dashboard_token_file)
    return settings


def vm_file(name):
    with open(os.path.join(VM_DIR, name), "rb") as handle:
        return handle.read()


def fail(message):
    print(message, file=sys.stderr)
    sys.exit(1)


def main_with(parser, handlers):
    """Run the subcommand the user picked, e.g. `up` or `down`."""
    args = parser.parse_args()
    handlers[args.command](args)
