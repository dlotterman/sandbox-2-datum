"""Create the exe.dev VM that runs the outside agent (public ghcr image, pinned by digest).
Usage: deploy-exe.py sha256:<digest> [port] [vm-name]   (port = host port to publish the dashboard on; exe.dev's proxy defaults to 8000)
Needs .env (EXEDEV), keys/exe.key, keys/exe.token, keys/inside.id. Secrets go into the VM's setup script as docker -e
flags. The EXEDEV token can only run new/ls/whoami/share show, so it cannot delete the VM or change the proxy."""
import sys, urllib.request, urllib.error

digest = sys.argv[1]
port = sys.argv[2] if len(sys.argv) > 2 else "8000"
name = sys.argv[3] if len(sys.argv) > 3 else "a2a-outside"
env = {}
for l in open(".env"):
    l = l.strip()
    if l and not l.startswith("#") and "=" in l:
        k, v = l.split("=", 1); env[k.strip()] = v.strip().strip("\"'")
rd = lambda f: open(f).read().strip()
script = (
    "#!/bin/bash\n"
    "for i in $(seq 1 40); do sudo docker info >/dev/null 2>&1 && break; sleep 5; done\n"
    "sudo docker rm -f a2a-outside >/dev/null 2>&1\n"
    f"sudo docker run -d --name a2a-outside --restart=always -p {port}:8080 "
    f"-e OUTSIDE_SECRET_KEY_HEX={rd('keys/exe.key')} -e INSIDE_ENDPOINT_ID={rd('keys/inside.id')} "
    f"-e DASHBOARD_TOKEN={rd('keys/exe.token')} -e REFRESH_SECS=600 ghcr.io/dlotterman/sandbox-2-datum@{digest}\n"
)
assert "'" not in script
cmd = f"new --name={name} --no-email --json --comment='A2A outside agent demo (Iroh)' --setup-script='" + script.replace("\n", "\\n") + "'"
req = urllib.request.Request("https://exe.dev/exec", data=cmd.encode(), headers={"Authorization": "Bearer " + env["EXEDEV"]}, method="POST")
try:
    r = urllib.request.urlopen(req, timeout=60); print(r.status, r.read().decode())
except urllib.error.HTTPError as e:
    print(e.code, e.read().decode())
