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
```bash
 docker build -t idsfinal .
```

## Implement CI/CD Pipeline: 

Establish a Continuous Integration and Continuous Deployment (CI/CD) pipeline to automate the testing, building, and deployment processes of the application. This pipeline should support rapid iteration and deployment of changes to the service.

