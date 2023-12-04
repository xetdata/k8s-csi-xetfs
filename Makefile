
lint:
	cd rust ; cargo clippy -r -- -D warnings

build-rust: rust
	cd builder ; docker-compose build rust_build ; docker-compose up rust_build

build-plugin: build-rust
	docker build -f builder/Dockerfile -t xet-csi-plugin:latest builder

build-counter-app: example
	docker build -f example/Dockerfile.app -t counter-app:latest example

run: build-plugin build-counter-app
	minikube image load xet-csi-plugin:latest
	minikube image load counter-app:latest
	# uncomment following if you have a secret to use
	#kubectl apply -f deploy/.secret.yaml
	kubectl apply -f deploy/csidriver.yaml
	kubectl apply -f deploy/node-serviceaccount.yaml
	kubectl apply -f deploy/node-daemonset.yaml
	kubectl apply -f example/apps.yaml

clean:
	kubectl delete --all --force pods
	kubectl delete --all --force daemonset
	sleep 1
	minikube image rm xet-csi-plugin:latest
