#!/bin/bash

# Copyright 2020 The Kubernetes Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Adapted from https://github.com/kubernetes-sigs/blob-csi-driver/blob/5a56e7600a67ae6b275b70c25ce3ca9c0bd61abe/deploy/install-driver.sh

set -euo pipefail

# the install script supports installing different versions
# but currently only master version is made available
ver="master"
if [[ "$#" -gt 0 ]]; then
  ver="$1"
fi

# TODO: this github link will not work until we make k8s-csi-xetfs repo public
repo="https://raw.githubusercontent.com/xetdata/k8s-csi-xetfs/$ver/deploy"
if [[ "$#" -gt 1 ]]; then
  if [[ "$2" == *"local"* ]]; then
    echo "use local deploy"
    repo="./deploy"
  fi
fi

if [ $ver != "master" ]; then
  repo="$repo/$ver"
fi

echo "Installing XetHub Storage CSI driver, version: $ver ..."
kubectl apply -f $repo/csidriver.yaml
kubectl apply -f $repo/node-daemonset.yaml
kubectl apply -f $repo/node-serviceaccount.yaml

echo 'XetHub Storage CSI driver installed successfully.'
