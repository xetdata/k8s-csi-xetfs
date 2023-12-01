# k8s-csi-xetfs
Kubernetes Container Storage Interface Plugin for XetHub. This plugin allows mounting and accessing XetHub repo by implementing [CSI specification](https://github.com/container-storage-interface/spec/blob/master/spec.md)

**csi plugin name: `csi.xethub.xetdata.com`**

## Project status: alpha

## Container Images & Kubernetes Compatibility
| driver version | Image                            | supported k8s version |
|----------------|----------------------------------|-----------------------|
| master branch  | <TODO: add link to docker image> | 1.25+                 |

## Features
Since this plugin is in alpha version, it supports minimal set of features provided by CSI spec.
- Single-pod read-only access mode
- Only supports [CSI ephemeral storage](https://kubernetes.io/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)

## Driver Parameters
| Name                      | Meaning                                                                                                                                            | Example                                 | Mandatory | Default value |
|---------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------|-----------|---------------|
| volumeAttributes.repo     | URL to the repository to mount                                                                                                                     | https://xethub.com/XetHub/Flickr30k.git | Yes       |               |
| volumeAttributes.commit   | Commit SHA or branch name to mount from the repo                                                                                                   | main                                    | Yes       |               |
| nodePublishSecretRef.name | Secret name that stores `user` and `pat` value for mounting private repo (See [secret.yaml.template](deploy/secret.yaml.template) for more details | existing Kubernetes secret name         | No        |               |

## Install driver on a Kubernetes cluster
### Install with kubectl
#### Option 1: remote install
```bash
curl -skSL https://raw.githubusercontent.com/xetdata/k8s-csi-xetfs/master/deploy/install-driver.sh | bash -s master blobfuse-proxy --
```
