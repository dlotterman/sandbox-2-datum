#!/bin/sh
# Install the outside agent on a plain Linux machine (a VM, a Fly Sprite, ...) from its container image.
#
# There is no container runtime here: unpack-image.py (next to this script) pulls the one binary out of the public
# image, and this script wraps it as a service. Run as root. Configuration comes from environment variables:
#
#   A2A_IMAGE             image to unpack, pinned by digest: ghcr.io/OWNER/REPO@sha256:...        (required)
#   INSIDE_ENDPOINT_ID    the inside agent's endpoint id                                          (required)
#   ENROLLMENT_TOKEN      the inside agent's enrollment token, so this agent needs no key        (one of these two)
#   OUTSIDE_SECRET_KEY_HEX  a pre-registered key, if the inside agent uses an allowlist instead   (one of these two)
#   DASHBOARD_TOKEN       optional token for the dashboard
#   HTTP_BIND             where the dashboard listens (default 127.0.0.1:8080, i.e. not exposed; "off" disables it)
#
# Afterwards: /usr/local/bin/outside-agent is the binary, /etc/a2a/outside.env holds the settings (root-only), and
# /usr/local/bin/a2a-outside-run starts the agent with them. If systemd is running it is also installed and started
# as the service a2a-outside; otherwise start the wrapper however your platform runs long-lived processes.
set -eu

: "${A2A_IMAGE:?set A2A_IMAGE to the image reference}"
: "${INSIDE_ENDPOINT_ID:?set INSIDE_ENDPOINT_ID}"
if [ -z "${ENROLLMENT_TOKEN:-}" ] && [ -z "${OUTSIDE_SECRET_KEY_HEX:-}" ]; then
  echo "set ENROLLMENT_TOKEN, or OUTSIDE_SECRET_KEY_HEX for a pre-registered key" >&2
  exit 1
fi

# Writing to /usr/local and /etc needs root. Where we start as a normal user with sudo (a Fly Sprite), run again under
# sudo and carry the settings across.
if [ "$(id -u)" != 0 ]; then
  exec sudo env "A2A_IMAGE=$A2A_IMAGE" "INSIDE_ENDPOINT_ID=$INSIDE_ENDPOINT_ID" "ENROLLMENT_TOKEN=${ENROLLMENT_TOKEN:-}" \
    "OUTSIDE_SECRET_KEY_HEX=${OUTSIDE_SECRET_KEY_HEX:-}" "DASHBOARD_TOKEN=${DASHBOARD_TOKEN:-}" "HTTP_BIND=${HTTP_BIND:-}" \
    sh "$0"
fi

HERE=$(cd "$(dirname "$0")" && pwd)
BINARY=/usr/local/bin/outside-agent
WRAPPER=/usr/local/bin/a2a-outside-run
ENV_FILE=/etc/a2a/outside.env

# 1. The unpacker needs python3 and CA certificates. Cloud images usually have both; install them if not.
if ! command -v python3 >/dev/null 2>&1 || [ ! -e /etc/ssl/certs/ca-certificates.crt ]; then
  apt-get update -qq
  apt-get install -y -qq python3 ca-certificates
fi

# 2. The binary, straight from the image's last layer (integrity-checked against the pinned digest).
python3 "$HERE/unpack-image.py" "$A2A_IMAGE" usr/local/bin/outside-agent "$BINARY"

# 3. Settings, in a file only root can read: they include secrets.
mkdir -p "$(dirname "$ENV_FILE")"
(
  umask 077
  {
    echo "INSIDE_ENDPOINT_ID=$INSIDE_ENDPOINT_ID"
    echo "HTTP_BIND=${HTTP_BIND:-127.0.0.1:8080}"
    [ -z "${ENROLLMENT_TOKEN:-}" ] || echo "ENROLLMENT_TOKEN=$ENROLLMENT_TOKEN"
    [ -z "${OUTSIDE_SECRET_KEY_HEX:-}" ] || echo "OUTSIDE_SECRET_KEY_HEX=$OUTSIDE_SECRET_KEY_HEX"
    [ -z "${DASHBOARD_TOKEN:-}" ] || echo "DASHBOARD_TOKEN=$DASHBOARD_TOKEN"
  } > "$ENV_FILE"
)

# Without systemd the agent runs as whoever started the install (not root), and must be able to read its settings.
if [ ! -d /run/systemd/system ] && [ -n "${SUDO_USER:-}" ]; then
  chown "$SUDO_USER" "$ENV_FILE"
fi

# 4. A wrapper that loads those settings and runs the agent, for platforms without systemd.
cat > "$WRAPPER" <<WRAP
#!/bin/sh
set -a
. $ENV_FILE
set +a
exec $BINARY
WRAP
chmod 0755 "$WRAPPER"

# 5. Run it as a service if systemd is available, as an unprivileged user, restarting on failure.
if [ -d /run/systemd/system ]; then
  id a2a >/dev/null 2>&1 || useradd --system --no-create-home --shell /usr/sbin/nologin a2a
  cat > /etc/systemd/system/a2a-outside.service <<UNIT
[Unit]
Description=A2A outside agent
After=network-online.target
Wants=network-online.target

[Service]
User=a2a
EnvironmentFile=$ENV_FILE
ExecStart=$BINARY
Restart=always
RestartSec=5
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes

[Install]
WantedBy=multi-user.target
UNIT
  systemctl daemon-reload
  systemctl enable --now a2a-outside
  echo "a2a-outside is running under systemd"
else
  echo "installed; no systemd here, so start $WRAPPER yourself"
fi
