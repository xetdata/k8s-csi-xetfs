
lint:
	cd rust ; cargo clippy -r -- -D warnings

build-rust: rust
	cd rust ; cargo build --release

build-plugin: build-rust
	docker build -f deploy/Dockerfile -t xet-csi-plugin:latest .


run: build-plugin
	minikube image load xet-csi-plugin:latest
	# uncomment following if you have a secret to use
	#kubectl apply -f deploy/.secret.yaml
	kubectl apply -f deploy/csidriver.yaml
	kubectl apply -f deploy/node-serviceaccount.yaml
	kubectl apply -f deploy/node-daemonset.yaml
	kubectl apply -f deploy/apps.yaml

clean:
	kubectl delete --all --force pods
	kubectl delete --all --force daemonset
	sleep 1
	minikube image rm xet-csi-plugin:latest
