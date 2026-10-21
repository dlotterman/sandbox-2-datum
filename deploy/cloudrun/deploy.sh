#!/usr/bin/env bash
# Deploy the outside agent to Cloud Run (source deploy of a bundle: Dockerfile + prebuilt binary).
# Always-on CPU and exactly one instance, because the agent runs background loops (registry sync every 30 s).
# Secrets (agent key, dashboard token) come from Secret Manager: a2a-run-key, a2a-run-token.
# Usage: BUNDLE=<dir with Dockerfile and outside-agent> deploy.sh
set -euo pipefail
cd "$(dirname "$0")/../.."
P=${GCP_PROJECT:?set GCP_PROJECT}; R=${GCP_REGION:-us-west1}
gcloud run deploy a2a-outside --project "$P" --region "$R" --source "${BUNDLE:?set BUNDLE}" --quiet \
  --execution-environment gen2 --no-cpu-throttling --min-instances 1 --max-instances 1 \
  --cpu 1 --memory 512Mi --port 8080 --allow-unauthenticated \
  --set-env-vars "INSIDE_ENDPOINT_ID=$(cat keys/inside.id),REFRESH_SECS=600" \
  --set-secrets "OUTSIDE_SECRET_KEY_HEX=a2a-run-key:latest,DASHBOARD_TOKEN=a2a-run-token:latest"
