//! The Postgres connection, schema, seed data and audit log.
//!
//! Connections are opened per operation. That is plenty for this workload and means a database restart
//! never leaves a stale connection behind.
use std::time::Duration;

use tokio_postgres::{Client, NoTls};

use crate::app::App;

/// How long to wait for the database, and how long between attempts while it is still starting.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const RETRY_DELAY: Duration = Duration::from_secs(10);

/// One row of the `platforms` reference table.
struct Platform {
    name: &'static str,
    kind: &'static str,
    signals: &'static str,
    regions: &'static str,
    notes: &'static str,
}

/// Demo reference data: typical runtime signals of hosting platforms. Hints, not proof.
const PLATFORMS: [Platform; 10] = [
    Platform {
        name: "Fly.io",
        kind: "microVM",
        signals: "env FLY_APP_NAME, FLY_REGION, FLY_MACHINE_ID; Firecracker; 6PN fdaa::/16",
        regions: "lax, sjc, iad, ams, ...",
        notes: "Long-lived machines, anycast edge",
    },
    Platform {
        name: "Railway",
        kind: "container",
        signals: "env RAILWAY_ENVIRONMENT, RAILWAY_PROJECT_ID, RAILWAY_SERVICE_ID",
        regions: "multiple",
        notes: "Git-driven deploys",
    },
    Platform {
        name: "RunPod",
        kind: "container (GPU)",
        signals: "env RUNPOD_POD_ID, RUNPOD_DC_ID; NVIDIA devices",
        regions: "US-CA-2, EU-RO-1, ...",
        notes: "GPU pods",
    },
    Platform {
        name: "Render",
        kind: "container",
        signals: "env RENDER=true, RENDER_SERVICE_ID",
        regions: "oregon, ohio, frankfurt, ...",
        notes: "Web services and workers",
    },
    Platform {
        name: "Heroku",
        kind: "container (dyno)",
        signals: "env DYNO, PORT; UUID-like hostname",
        regions: "us, eu",
        notes: "Ephemeral dynos",
    },
    Platform {
        name: "AWS ECS/Fargate",
        kind: "microVM",
        signals: "env ECS_CONTAINER_METADATA_URI_V4; DMI Amazon EC2",
        regions: "any AWS region",
        notes: "Firecracker-backed Fargate",
    },
    Platform {
        name: "Kubernetes",
        kind: "container",
        signals: "env KUBERNETES_SERVICE_HOST; cgroup kubepods",
        regions: "n/a",
        notes: "Any distribution",
    },
    Platform {
        name: "Datum Compute",
        kind: "container / unikernel",
        signals: "IPv6-only interface on fd20::/48 private network",
        regions: "us-west-1, us-central-1, us-east-1",
        notes: "Instances join a Datum network",
    },
    Platform {
        name: "Local Podman (rootless)",
        kind: "container",
        signals: "/run/.containerenv, no /.dockerenv, 10.89.x.x bridge",
        regions: "n/a",
        notes: "Home or office machine",
    },
    Platform {
        name: "Local Docker",
        kind: "container",
        signals: "/.dockerenv, 172.17.x.x bridge",
        regions: "n/a",
        notes: "Home or office machine",
    },
];

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS platforms (
        name text PRIMARY KEY, kind text, signals text, regions text, notes text
    );
    CREATE TABLE IF NOT EXISTS agents_seen (
        endpoint_id text PRIMARY KEY,
        first_seen  timestamptz NOT NULL DEFAULT now(),
        last_seen   timestamptz NOT NULL DEFAULT now(),
        requests    bigint NOT NULL DEFAULT 0,
        report      text,
        reported_at timestamptz
    );
    CREATE TABLE IF NOT EXISTS agent_audit (
        id bigserial PRIMARY KEY,
        at timestamptz NOT NULL DEFAULT now(),
        peer text, skill text, decision text, reason text
    );";

/// Open a connection. The error is plain text, ready to wrap in whatever error type the caller uses.
pub async fn connect(app: &App) -> Result<Client, String> {
    let (client, connection) = tokio_postgres::Config::new()
        .host(&app.postgres.host)
        .user(&app.postgres.user)
        .password(&app.postgres.password)
        .dbname(&app.postgres.database)
        .connect_timeout(CONNECT_TIMEOUT)
        .connect(NoTls)
        .await
        .map_err(|e| format!("postgres connect: {e}"))?;
    // tokio-postgres needs its connection driven on a task of its own.
    tokio::spawn(async move {
        let _ = connection.await;
    });
    Ok(client)
}

/// Create the tables and load the reference data, retrying until Postgres is reachable. Called once at
/// startup in the background: the database often finishes starting after this agent does.
pub async fn init(app: &App) {
    if !app.postgres.is_configured() {
        eprintln!("POSTGRES_HOST not set; data skills disabled");
        return;
    }
    for attempt in 1.. {
        match create_schema_and_seed(app).await {
            Ok(()) => return eprintln!("postgres ready (seeded)"),
            Err(e) => {
                eprintln!("postgres init attempt {attempt}: {e}");
                tokio::time::sleep(RETRY_DELAY).await;
            }
        }
    }
}

async fn create_schema_and_seed(app: &App) -> Result<(), String> {
    let client = connect(app).await?;
    client.batch_execute(SCHEMA).await.map_err(|e| e.to_string())?;
    for platform in &PLATFORMS {
        client
            .execute(
                "INSERT INTO platforms VALUES ($1, $2, $3, $4, $5) ON CONFLICT (name) DO NOTHING",
                &[&platform.name, &platform.kind, &platform.signals, &platform.regions, &platform.notes],
            )
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Write one policy decision to the audit table. Best effort: an unreachable database must never
/// block or fail the request being audited, so errors are dropped.
pub async fn record_audit(app: &App, peer: &str, skill: &str, decision: &str, reason: &str) {
    if !app.postgres.is_configured() {
        return;
    }
    if let Ok(client) = connect(app).await {
        let _ = client
            .execute(
                "INSERT INTO agent_audit (peer, skill, decision, reason) VALUES ($1, $2, $3, $4)",
                &[&peer, &skill, &decision, &reason],
            )
            .await;
    }
}
