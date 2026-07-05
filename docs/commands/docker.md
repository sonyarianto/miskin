# Docker / Kubernetes Commands

## docker ps

```bash
$ miskin docker ps

2 entries
abc123  nginx:latest  Up 2h
def456  redis:alpine  Up 5m
```

Empty:

```bash
empty
```

## docker images

```bash
$ miskin docker images

3 entries
alpine   latest    abc123   5MB
nginx    latest    def456   190MB
```

## docker logs

Deduplicates repeated log lines:

```bash
$ miskin docker logs myapp

[INFO] Server started on :8080
[WARN] Rate limit approaching
[ERROR] Connection timeout to db:5432 (repeated 42x)
```

## docker compose

```bash
$ miskin docker compose ps

ok
```

## kubectl

Kubernetes commands use the same filter as Docker:

```bash
$ miskin kubectl get pods

3 entries
myapp-abc123   Running   2d
myapp-def456   Running   5h
```

## oc (OpenShift)

```bash
$ miskin oc get pods
$ miskin oc get services
$ miskin oc logs myapp
```
