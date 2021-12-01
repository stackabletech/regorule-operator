#!/bin/bash
# This script reads a Helm chart from deploy/helm/regorule-operator and
# generates manifest files into deploy/manifestss
set -e

tmp=$(mktemp -d ./manifests-XXXXX)

helm template --output-dir $tmp \
              --include-crds \
              --name-template regorule-operator \
              deploy/helm/regorule-operator

for file in $(find $tmp -type f)
do
    yq eval -i 'del(.. | select(has("app.kubernetes.io/managed-by")) | ."app.kubernetes.io/managed-by")' $file
    yq eval -i 'del(.. | select(has("helm.sh/chart")) | ."helm.sh/chart")' $file
    sed -i '/# Source: .*/d' $file
done

cp -r $tmp/regorule-operator/*/* deploy/manifests/

rm -rf $tmp
