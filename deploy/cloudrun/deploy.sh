#!/usr/bin/env bash
# Deploy the outside agent to Cloud Run: a source deploy of a bundle (Dockerfile + prebuilt binary).
# CPU is always allocated and there is exactly one instance, because the agent runs background loops.
# Its secrets live in Secret Manager and are created or updated from files by this script.
#
#   GCP_PROJECT=my-project SERVICE=a2a-outside-b BIN=path/to/outside-agent INSIDE_ID_FILE=keys/inside-b.id \
#     ENROLLMENT_TOKEN_FILE=keys/enrollment-b.token DASHBOARD_TOKEN_FILE=keys/cloudrun-b.token  deploy/cloudrun/deploy.sh
#
#   GCP_PROJECT            project id (required; no default so nothing deploys to the wrong place)
#   GCP_REGION             default us-west1
#   SERVICE                Cloud Run service name, also the prefix of its secrets (default a2a-outside)
#   BIN                    the built outside-agent binary                                       (required)
#   INSIDE_ID_FILE         file with the inside agent's endpoint id                             (required)
#   ENROLLMENT_TOKEN_FILE  the inside agent's enrollment token: no key needed (preferred)       (one of these two)
#   KEY_FILE               a pre-registered secret key (hex) instead                            (one of these two)
#   DASHBOARD_TOKEN_FILE   token protecting the public dashboard                                (required)
#
# Needs gcloud, logged in, with the Cloud Run, Cloud Build, Artifact Registry and Secret Manager APIs enabled.
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${GCP_PROJECT:?set GCP_PROJECT}"
: "${BIN:?set BIN to the path of the built outside-agent binary}"
: "${INSIDE_ID_FILE:?set INSIDE_ID_FILE}"
: "${DASHBOARD_TOKEN_FILE:?set DASHBOARD_TOKEN_FILE}"
[ -n "${ENROLLMENT_TOKEN_FILE:-}${KEY_FILE:-}" ] || { echo "set ENROLLMENT_TOKEN_FILE (or KEY_FILE)" >&2; exit 1; }
REGION=${GCP_REGION:-us-west1}
SERVICE=${SERVICE:-a2a-outside}

WORK=$(mktemp -d); trap 'rm -rf "$WORK"' EXIT
cp deploy/Containerfile.outside "$WORK/Dockerfile"
cp "$BIN" "$WORK/outside-agent"

# Create the secret, or add a new version if it exists. printf %s: no trailing newline gets stored.
put_secret() {
  local name=$1 file=$2
  if gcloud secrets describe "$name" --project "$GCP_PROJECT" >/dev/null 2>&1; then
    printf %s "$(cat "$file")" | gcloud secrets versions add "$name" --project "$GCP_PROJECT" --data-file=- >/dev/null
  else
    printf %s "$(cat "$file")" | gcloud secrets create "$name" --project "$GCP_PROJECT" --replication-policy=automatic --data-file=- >/dev/null
  fi
  gcloud secrets add-iam-policy-binding "$name" --project "$GCP_PROJECT" --member="serviceAccount:$RUNTIME_SA" \
    --role=roles/secretmanager.secretAccessor --format="value(etag)" >/dev/null
}
RUNTIME_SA="$(gcloud projects describe "$GCP_PROJECT" --format='value(projectNumber)')-compute@developer.gserviceaccount.com"

SECRETS="DASHBOARD_TOKEN=$SERVICE-dashboard:latest"
put_secret "$SERVICE-dashboard" "$DASHBOARD_TOKEN_FILE"
if [ -n "${ENROLLMENT_TOKEN_FILE:-}" ]; then
  put_secret "$SERVICE-enrollment" "$ENROLLMENT_TOKEN_FILE"; SECRETS="$SECRETS,ENROLLMENT_TOKEN=$SERVICE-enrollment:latest"
else
  put_secret "$SERVICE-key" "$KEY_FILE"; SECRETS="$SECRETS,OUTSIDE_SECRET_KEY_HEX=$SERVICE-key:latest"
fi
sleep 3

gcloud run deploy "$SERVICE" --project "$GCP_PROJECT" --region "$REGION" --source "$WORK" --quiet \
  --execution-environment gen2 --no-cpu-throttling --min-instances 1 --max-instances 1 \
  --cpu 1 --memory 512Mi --port 8080 --allow-unauthenticated \
  --set-env-vars "INSIDE_ENDPOINT_ID=$(cat "$INSIDE_ID_FILE"),REFRESH_SECS=600" \
  --set-secrets "$SECRETS"
