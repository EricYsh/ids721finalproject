# LLMOps - Model Serving with Rust

> IDS 721 Spring 2024 Final Project
> By Team 24
> Members: Zeyu Yuan, Shanghui Yin, Yang Ouyang, Guowang Zeng


## Requirements:

## Obtain Open Source ML Model: 

Select and acquire an open-source machine learning model suitable for serving. The model should be capable of providing inferences based on input data it receives. Based on the requirements, we decided to use `Falconsai/text_summarization` from HuggingFace.

Due to the nature of HuggingFace's transformers and pipeline, we decided to use python as model

## Create Rust Web Service for Model Inferences: 

Develop a web service in Rust that can serve the ML model's inferences. The service should be robust, efficiently handling requests and providing accurate responses from the model.

## Containerize Service and Deploy to Kubernetes: 

Containerize the Rust web service using Docker, ensuring it's well-prepared for deployment. Subsequently, deploy the containerized service to a Kubernetes cluster, configuring it for scalability and reliability.

Build the Rust web service into a Docker container, ensuring it's well-prepared for deployment. Subsequently, deploy the containerized service to a Kubernetes cluster, configuring it for scalability and reliability.

### Build the docker file
```bash
 docker build -t test-ecr .
```

### Run the docker container
```bash
docker run -p 8080:8080 test-ecr
```

### Push the docker image to ECR
```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 123456789012.dkr.ecr.us-east-1.amazonaws.com
docker tag test-ecr:latest 123456789012.dkr.ecr.us-east-1.amazonaws.com/test-ecr:latest
docker push 123456789012.dkr.ecr.us-east-1.amazonaws.com/test-ecr:latest
```

### Deploy the docker image to EKS

#### Pre-requisites
- install eksctl
- install kubectl
- install awscli

#### Create EKS cluster
```bash
eksctl create cluster --name test-eks --region us-east-2
```


```bash
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```


## Implement CI/CD Pipeline: 

A gitlab `.gitlab-ci.yml` file is included in the repository. The pipeline is configured to build the Rust web service, containerize it, and deploy it to a Kubernetes cluster. The pipeline should be triggered automatically upon pushing changes to the repository.
