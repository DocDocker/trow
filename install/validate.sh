#!/bin/bash
set -eo pipefail
unset CDPATH
IFS=$'\n\t'

echo "Setting up trow as a validating webhook"
echo "WARNING: This will limit what images can run in your cluster"
echo "By default, only images in Trow and official Kubernetes images will be 
allowed"
echo

cabundle=$(kubectl get secret -o jsonpath="{.items[?(@.type==\"kubernetes.io/service-account-token\")].data['ca\.crt']}")
#Really not happy about use of sed here
sed "s/{{cabundle}}/${cabundle}/" validate-tmpl.yaml > validate.yaml
kubectl apply -f validate.yaml
