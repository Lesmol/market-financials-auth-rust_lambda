# 🛡️ Market Financials Auth Rust Lambda

This repository contains an AWS Lambda function implemented in Rust for API authentication. It validates API keys against a DynamoDB table to authorize requests. The function is built using Cargo Lambda and deployed via GitHub Actions.

## 📂 Repository Structure

* **`src/main.rs`**: The entry point that initializes the DynamoDB client and starts the Lambda runtime.
* **`src/http_handler.rs`**: Contains the core logic for extracting the API key and querying DynamoDB.
* **`src/auth_response.rs`**: Defines the structure for the JSON authorization response.
* **`.github/workflows/build-deploy.yml`**: CI/CD pipeline for building the Rust binary and deploying the Lambda function.

## 🏗️ Architecture & Logic

The Lambda function expects an HTTP event and specifically looks for an `x-api-key` header to validate authorization.

1.  **📥 Input:** The handler extracts the `x-api-key` header from the incoming request.
2.  **🔍 Validation:**
    * It checks for the presence of the header.
    * If present, it queries the DynamoDB table (defined by the `DYNAMODB_AUTH_TABLE_NAME` environment variable) to see if the key exists as a primary key (`ApiKey`).
3.  **📤 Output:** Returns a JSON object indicating authorization status:
    * `{ "isAuthorized": true }` if the key exists.
    * `{ "isAuthorized": false }` if the key is missing or invalid.

## 🚀 Deployment

Deployment is handled automatically via GitHub Actions when changes are pushed to the `develop` branch.

### 🔄 CI/CD Pipeline
The workflow `build-deploy.yml` performs the following steps:
1.  **Build**:
    * Sets up the Rust toolchain and `cargo-lambda`.
    * Compiles the function in release mode (`cargo lambda build --release`).
    * Uploads the build artifact.
2.  **Deploy**:
    * Downloads the artifact.
    * Deploys the function to AWS using `cargo lambda deploy`.

### ⚙️ Environment Configuration
The deployment pipeline uses the following AWS configuration:
* **🌍 Region:** `af-south-1`
* **⚡ Lambda Function Name:** Configured via the `LAMBDA_FUNCTION_NAME` secret.

## ✅ Requirements

* **🔑 AWS Credentials**: The GitHub repository secrets must include `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` for deployment.
* **📝 Environment Variables**: The Lambda function requires `DYNAMODB_AUTH_TABLE_NAME` to be set in the runtime environment.

## 🔨 Tools used:
[![My Skills](https://skillicons.dev/icons?i=rust,aws,githubactions&perline=6)](https://skillicons.dev)