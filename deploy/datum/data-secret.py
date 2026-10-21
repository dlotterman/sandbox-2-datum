"""Print the a2a-data-env Secret manifest (random credentials) for `datumctl apply -f -`.
Used by the Postgres, rclone and inside-agent workloads. Re-running rotates the values."""
import json
import secrets

data = {
    "POSTGRES_USER": "a2a",
    "POSTGRES_PASSWORD": secrets.token_urlsafe(24),
    "POSTGRES_DB": "a2a",
    "RC_USER": "a2a",
    "RC_PASS": secrets.token_urlsafe(24),
}
print(json.dumps({
    "apiVersion": "v1", "kind": "Secret",
    "metadata": {"name": "a2a-data-env"}, "type": "Opaque", "stringData": data,
}))
