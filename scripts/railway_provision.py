"""UNTESTED END TO END: Railway's API was down (platform outage) when this was written, so the later steps never ran.
Create the Railway project/service/variables/domain for the outside agent (idempotent by project name).
Reads RAILWAY_TOKEN from .env; prints ids only, never secrets. Output: project, environment, service, domain."""
import json, sys, urllib.request

env = {}
for line in open(".env"):
    line = line.strip()
    if line and not line.startswith("#") and "=" in line:
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip("\"'")

def gql(query, variables=None):
    req = urllib.request.Request(
        "https://backboard.railway.com/graphql/v2",
        data=json.dumps({"query": query, "variables": variables or {}}).encode(),
        headers={"Authorization": "Bearer " + env["RAILWAY_TOKEN"], "Content-Type": "application/json", "User-Agent": "sandbox-2-datum/0.1"},
    )
    d = json.loads(urllib.request.urlopen(req, timeout=40).read())
    if d.get("errors"):
        sys.exit("graphql error: " + json.dumps(d["errors"])[:400])
    return d["data"]

NAME = "a2a-outside"
me = gql("{ me { workspaces { id name } } }")["me"]
ws = me["workspaces"][0]
print("workspace:", ws["name"], file=sys.stderr)

projects = gql("{ me { workspaces { id projects { edges { node { id name environments { edges { node { id name } } } services { edges { node { id name } } } } } } } } }")
found = None
for w in projects["me"]["workspaces"]:
    for e in w["projects"]["edges"]:
        if e["node"]["name"] == NAME:
            found = e["node"]
if found:
    proj = found
else:
    proj = gql("""mutation($i: ProjectCreateInput!) { projectCreate(input: $i) { id name
        environments { edges { node { id name } } } services { edges { node { id name } } } } }""",
        {"i": {"name": NAME, "workspaceId": ws["id"]}})["projectCreate"]
pid = proj["id"]
envid = proj["environments"]["edges"][0]["node"]["id"]
svc = next((e["node"] for e in proj["services"]["edges"] if e["node"]["name"] == NAME), None)
if not svc:
    svc = gql("mutation($i: ServiceCreateInput!) { serviceCreate(input: $i) { id name } }",
              {"i": {"projectId": pid, "name": NAME}})["serviceCreate"]
sid = svc["id"]

variables = {
    "OUTSIDE_SECRET_KEY_HEX": open("keys/railway.key").read().strip(),
    "INSIDE_ENDPOINT_ID": open("keys/inside.id").read().strip(),
    "DASHBOARD_TOKEN": open("keys/railway.token").read().strip(),
    "PORT": "8080",
    "REFRESH_SECS": "600",
}
gql("mutation($i: VariableCollectionUpsertInput!) { variableCollectionUpsert(input: $i) }",
    {"i": {"projectId": pid, "environmentId": envid, "serviceId": sid, "variables": variables}})

doms = gql("query($p: String!, $e: String!, $s: String!) { domains(projectId: $p, environmentId: $e, serviceId: $s) { serviceDomains { domain } } }",
           {"p": pid, "e": envid, "s": sid})["domains"]["serviceDomains"]
if doms:
    domain = doms[0]["domain"]
else:
    domain = gql("mutation($i: ServiceDomainCreateInput!) { serviceDomainCreate(input: $i) { domain } }",
                 {"i": {"serviceId": sid, "environmentId": envid, "targetPort": 8080}})["serviceDomainCreate"]["domain"]
print(json.dumps({"project": pid, "environment": envid, "service": sid, "domain": domain}))
