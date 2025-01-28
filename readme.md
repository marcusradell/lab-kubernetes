# lab-kubernetes

Create a repo with three subfolders: /frontend, /backend, and /infrastructure.

## /frontend

Setup a vite project with react-ts as template.

## /backend

Setup a backend with a dockerfile that works for you locally via docker run.

## /infrastructure

### tools

- kubectl
- kustomize
- k9s
- minikube

### Minikube

Main commands:

`minikube start`
`minikube dashboard`
`minikube delete --all`

### Port Forwarding

Can be done with `k9s` or `kubectl port-forward service/jork-api 3000:3000`.
