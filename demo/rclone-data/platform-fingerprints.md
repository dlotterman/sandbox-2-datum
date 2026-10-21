# Hosting platform fingerprints (demo reference data)

Typical signals only. Treat as hints, not proof; verify against what the runtime actually shows.

- Fly.io: env FLY_APP_NAME, FLY_REGION, FLY_MACHINE_ID, FLY_ALLOC_ID. Runs in Firecracker microVMs; hostname is
  usually the machine id; private 6PN network uses fdaa::/16 addresses.
- Railway: env RAILWAY_ENVIRONMENT, RAILWAY_PROJECT_ID, RAILWAY_SERVICE_ID, RAILWAY_REPLICA_ID.
- RunPod: env RUNPOD_POD_ID, RUNPOD_DC_ID, RUNPOD_POD_HOSTNAME; GPU pods, often NVIDIA devices visible.
- Render: env RENDER=true, RENDER_SERVICE_ID, RENDER_EXTERNAL_URL.
- Heroku: env DYNO, PORT; ephemeral dynos with hostnames like a UUID.
- AWS ECS/Fargate: env ECS_CONTAINER_METADATA_URI_V4, AWS_EXECUTION_ENV; DMI may show Amazon EC2 / Firecracker.
- Kubernetes (any): env KUBERNETES_SERVICE_HOST, KUBERNETES_SERVICE_PORT; cgroup path contains kubepods.
- Datum Compute: instances attach to an IPv6-only network (private fd20::/48 prefixes); no IPv4 on the interface.
- Local rootless Podman: /run/.containerenv present, /.dockerenv absent, 10.89.x.x bridge addresses when using the
  default network, hostname is a 12-character container id.
- Local Docker: /.dockerenv present, 172.17.x.x bridge addresses by default.
- Daytona, exe.dev: no verified signals recorded here yet.

Residential vs datacenter: an ASN belonging to a consumer ISP (cable/fiber) plus DMI showing physical hardware vendors
(Apple, Dell, Lenovo, ...) points to a home or office machine; hypervisor vendors in DMI (QEMU, Amazon EC2, Google
Compute Engine, Microsoft Hyper-V) point to a cloud VM.
