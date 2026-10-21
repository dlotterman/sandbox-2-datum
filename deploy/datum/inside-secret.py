"""Print the a2a-inside-env Secret manifest for `datumctl apply -f -`.
Usage: inside-secret.py <datum-loreate .env> <inside.key> <outside.id>[,<outside.id>...]
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
}
print(json.dumps({
    "apiVersion": "v1", "kind": "Secret",
    "metadata": {"name": "a2a-inside-env"}, "type": "Opaque", "stringData": data,
}))
