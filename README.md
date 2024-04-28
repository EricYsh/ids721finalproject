# LLMOps - Model Serving with Rust

> IDS 721 Spring 2024 Final Project
> By Team 24
> Members: Zeyu Yuan, Shanghui Yin, Yang Ouyang, Guowang Zeng

## Demo Video

Here is our demo video: 

## Prerequisites

Ensure the following software is installed:

- Rust environment (including Cargo)
- Actix-web
- Python 3.9
- AWS CLI
- AWS ECR
- AWS EKS
- AWS IAM
- Docker, Docker Hub
- kubectl with Kubernetes cluster connection configured
- eksctl
- Github Actions
- HuggingFace transformers


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
kubectl create namespace eks-app
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml
```


## Implement CI/CD Pipeline: 

A github `cicd.yml` file is included in the repository. The pipeline is configured to build the Rust web service, containerize it, and deploy it to a Kubernetes cluster. The pipeline should be triggered automatically upon pushing changes to the repository. It has features such as environment variables, jobs, and triggers:
### Triggers:
- `push`: The workflow is triggered on a push to the main branch.
- `pull_request`: The workflow is also triggered on pull requests to the main branch.
### Environment Variables:
- Sets up several environment variables such as ECR_REPOSITORY, EKS_CLUSTER_NAME, and AWS_REGION which are used throughout the workflow.
### Jobs:
- `build`:
    - Runs on an Ubuntu latest virtual machine.
    - Uses Docker's dind (Docker in Docker) service to build a Docker image.
    - Logs into DockerHub and pushes a Docker image tagged as idsfinal.
- `test`:
    - Depends on the build job.
    - Uses a Rust environment for testing.
    - Sets up Python and installs dependencies.
    - Runs Rust tests specified by the command cargo test.
- `deploy`:
    - Depends on the test job.
    - Uses actions to configure AWS credentials, log into Amazon ECR, and push the Docker image to AWS's ECR.
    - Updates Kubernetes configuration to connect to an Amazon EKS cluster.
    - Deploys to the Amazon EKS cluster using Kubernetes deployment and service configuration files specified in eks-deployment.yaml and eks-service.yaml.
