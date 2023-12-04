# k8s-csi-xetfs
Kubernetes Container Storage Interface Plugin for XetHub. This plugin enables mounting and accessing XetHub repository from Kubernetes by implementing [CSI specification](https://github.com/container-storage-interface/spec/blob/master/spec.md)

### csi plugin name: `csi.xethub.xetdata.com`

### Project status: alpha

## Container Images & Kubernetes Compatibility
| driver version | Image                                 | supported k8s version |
|----------------|---------------------------------------|-----------------------|
| main branch    | docker.io/xethub/k8s-csi-xetfs:latest | 1.25+                 |

## Features
Since this plugin is in alpha version, it supports minimal set of features provided by CSI spec.
- Single-pod read-only access mode
- Only supports [CSI ephemeral storage](https://kubernetes.io/docs/concepts/storage/ephemeral-volumes/#csi-ephemeral-volumes)

## Example usage
This repo contains an example python application under [example](./example) directory that uses XetHub CSI driver to mount `Flickr30k` [repository](https://xethub.com/XetHub/Flickr30k), reads all the files and prints out the total size of the repo. Start the example application by running the following commands:

```bash
kubectl apply -f - <<EOF
---
apiVersion: v1
kind: Pod
metadata:
  name: app1
spec:
  containers:
    - name: app1
      image: docker.io/xethub/counter-app:latest
      volumeMounts:
        - name: xet-flickr-30
          mountPath: /data
  volumes:
    - name: xet-flickr-30
      csi:
        driver: csi.xethub.xetdata.com
        readOnly: true
        volumeAttributes:
          repo: https://xethub.com/XetHub/Flickr30k.git
          commit: main
          # following is setup for a private repo mount
          # requires a secret configured like in secret.yaml.template
          #- name: private-repo
          #csi:
          #driver: csi.xethub.xetdata.com
          #readOnly: true
          #volumeAttributes:
          #repo: <https url of private repo>
          #commit: main
          #nodePublishSecretRef:
          #name: <secret name>
EOF
```

## Driver Parameters
| Name                      | Meaning                                                                                                                                                  | Example                                 | Mandatory                                  | Default value |
|---------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------|--------------------------------------------|---------------|
| volumeAttributes.repo     | URL to the repository to mount                                                                                                                           | https://xethub.com/XetHub/Flickr30k.git | Yes                                        |               |
| volumeAttributes.commit   | Commit SHA or branch name to mount from the repository                                                                                                   | main                                    | Yes                                        |               |
| nodePublishSecretRef.name | Secret name that stores `user` and `pat` value for mounting private repository (See [secret.yaml.template](deploy/secret.yaml.template) for more details | existing Kubernetes secret name         | No (yes for mounting private repositories) |               |

## Install driver on a Kubernetes cluster via kubectl
### Option 1: remote install
```bash
curl -skSL https://raw.githubusercontent.com/xetdata/k8s-csi-xetfs/master/deploy/install-driver.sh | bash -s main --
```

### Option 2: local install
```bash
git clone git@github.com:xetdata/k8s-csi-xetfs.git
cd k8s-csi-xetfs
./deploy/install-driver.sh main local
```

### After installing
- Check pod status
  ```bash
  kubectl -n kube-system get pod -o wide -l app=  kubectl -n kube-system get pod -o wide -l app=csi-blob-node
  ```
- Example output
  ```bash
  NAME                 READY   STATUS    RESTARTS   AGE     IP             NODE    
  xet-csi-node-zvfj9   3/3     Running   0          5m59s   192.168.49.2   minikube
  ```

## Uninstall driver on a Kubernetes cluster
### Option 1: remote uninstall
```bash
curl -skSL https://raw.githubusercontent.com/xetdata/k8s-csi-xetfs/master/deploy/uninstall-driver.sh | bash -s main --
```

### Option 2: local uninstall
```bash
git clone git@github.com:xetdata/k8s-csi-xetfs.git
cd k8s-csi-xetfs
./deploy/uninstall-driver.sh main local
```
