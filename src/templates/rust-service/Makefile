.PHONY: build-lambda deploy-lambda clean test run build-all invoke-lambda fix-lambda-imports

# Default target
all: build-all

# Build the main server
build:
	cargo build --release

# Build the Lambda function
build-lambda:
	cargo lambda build --release --bin rust-qr-generator-lambda --target aarch64-unknown-linux-gnu

# Build both the server and Lambda function
build-all: build build-lambda

# Deploy the Lambda function to AWS
deploy-lambda: build-lambda
	cargo lambda deploy rust-qr-generator-lambda --iam-role arn:aws:iam::113762419383:role/cargo-lambda-role-fd9b232d-d856-4603-8628-d6e6d26c69a6

# Run the local server
run:
	cargo run --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Fix the lambda.rs imports issue
fix-lambda-imports:
	@echo "Fixing lambda.rs imports..."
	@sed -i '' 's/#\[tokio::main\]//' src/lambda.rs
	@sed -i '' 's/async fn main/pub async fn main/' src/lambda.rs

# Invoke the Lambda function locally
invoke-lambda:
	cargo lambda invoke --data-ascii "{\
                                       \"version\": \"2.0\",\
                                       \"rawPath\": \"/api/v1/scrape/health\",\
                                       \"requestContext\": {\
                                         \"apiId\": \"api-id\",\
                                         \"http\": {\
                                           \"method\": \"GET\",\
                                           \"protocol\": \"HTTP/1.1\",\
                                           \"userAgent\": \"agent\"\
                                         },\
                                         \"requestId\": \"1234\"\
                                       },\
                                       \"isBase64Encoded\": false\
                                     }" --output-format json

# Watch for changes and automatically rebuild Lambda
watch-lambda:
	cargo lambda watch

# Help target
help:
	@echo "Available targets:"
	@echo "  build         - Build the main server"
	@echo "  build-lambda  - Build the Lambda function"
	@echo "  build-all     - Build both server and Lambda"
	@echo "  deploy-lambda - Deploy Lambda to AWS"
	@echo "  run           - Run the local server"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  fix-lambda-imports - Fix lambda.rs import issues"
	@echo "  invoke-lambda - Invoke Lambda function locally"
	@echo "  watch-lambda  - Watch for changes and rebuild Lambda"
