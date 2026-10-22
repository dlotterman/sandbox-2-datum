"""Print the a2a-inside-env Secret manifest for `datumctl apply -f -`.
Usage: inside-secret.py <.env with BASETEN_API_KEY and BASETEN_MODEL> <inside.key> <allowed ids> [enrollment-token-file]
  allowed ids            comma-separated endpoint ids that are always allowed; may be "" if you use enrollment
  enrollment-token-file  optional: a file holding the shared token that lets outside agents enroll themselves
Only the Baseten credentials are taken from the .env; nothing else is copied."""
import json
import sys

env = {}
for line in open(sys.argv[1]):
    line = line.strip()
    if line and not line.startswith("#") and "=" in line:
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip("\"'")

data = {
    "BASETEN_API_KEY": env["BASETEN_API_KEY"],
    "BASETEN_MODEL": env["BASETEN_MODEL"],
    "INSIDE_SECRET_KEY_HEX": open(sys.argv[2]).read().strip(),
    "ALLOWED_PEERS": sys.argv[3],
    # Always present, because the workload reads it; empty means enrollment is off.
    "ENROLLMENT_TOKEN": open(sys.argv[4]).read().strip() if len(sys.argv) > 4 else "",
}
print(json.dumps({
    "apiVersion": "v1", "kind": "Secret",
    "metadata": {"name": "a2a-inside-env"}, "type": "Opaque", "stringData": data,
}))
