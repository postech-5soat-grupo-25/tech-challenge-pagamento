deploy-mongo:
	kubectl apply -f ./k8s/mongodb-secret.yaml
	kubectl apply -f ./k8s/mongodb-pvc.yaml
	kubectl apply -f ./k8s/mongodb-deployment.yaml
	kubectl apply -f ./k8s/mongodb-service.yaml
	kubectl apply -f ./k8s/mongo-express-deployment.yaml
	kubectl apply -f ./k8s/mongo-express-service.yaml

deploy-rabbit:
	-kubectl delete -f ./k8s/rabbitmq-deployment.yaml
	-kubectl delete -f ./k8s/rabbitmq-service.yaml
	kubectl apply -f ./k8s/rabbitmq-deployment.yaml
	kubectl apply -f ./k8s/rabbitmq-service.yaml

local-deploy:
	-kubectl delete -f ./k8s/deployment.yaml
	-kubectl delete -f ./k8s/services.yaml
	-kubectl delete -f ./k8s/configmap.yaml
	kubectl apply -f ./k8s/deployment.yaml
	kubectl apply -f ./k8s/services.yaml
	kubectl apply -f ./k8s/configmap.yaml
		

build-image:
	mvn clean install
	docker build -t mblopes/tech-challenge-pagamentos:latest .
	docker push mblopes/tech-challenge-pagamentos:latest
	docker rmi mblopes/tech-challenge-pagamentos:latest

delete-from-cluster:
	-kubectl delete -f ./k8s/deployment.yaml
	-kubectl delete -f ./k8s/services.yaml

full-local-deploy: build-image local-deploy
