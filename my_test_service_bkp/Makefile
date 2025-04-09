.PHONY: all build build-lambda build-all deploy-lambda clean test run invoke-lambda watch-lambda fix-lambda-imports

# Lambda binary name based on project name
LAMBDA_BINARY := my_test_service_lambda

# Default target
all: build-all

# Build the main server
build:
	cargo build --release --bin server

# Build the Lambda function
build-lambda:
	cargo lambda build --release --bin $(LAMBDA_BINARY) --target aarch64-unknown-linux-gnu
	@echo "Lambda binary built: $(LAMBDA_BINARY)"

# Build both the server and Lambda function
build-all: build build-lambda

# Deploy the Lambda function to AWS
deploy-lambda: build-lambda
	@echo "Deploying Lambda function $(LAMBDA_BINARY)..."
	cargo lambda deploy $(LAMBDA_BINARY) \
		--iam-role arn:aws:iam::113762419383:role/cargo-lambda-role-719aaec1-c039-4aaf-8053-86406b318c11

# Run the local server
run:
	cargo run --release --bin server

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/lambda

# Fix the lambda.rs imports issue
fix-lambda-imports:
	@echo "Fixing lambda.rs imports..."
	@sed -i '' 's/#\[tokio::main\]//' src/services/lambda.rs
	@sed -i '' 's/async fn main/pub async fn main/' src/services/lambda.rs

# Invoke the Lambda function locally
invoke-lambda:
	@echo "Invoking Lambda function $(LAMBDA_BINARY)..."
	cargo lambda invoke $(LAMBDA_BINARY) \
		--data-ascii '{"message": "test message"}' \
		--output-format json

# Watch for changes and automatically rebuild Lambda
watch-lambda:
	cargo lambda watch

# Help command
help:
	@echo "Available commands:"
	@echo "  make all           - Build everything (default)"
	@echo "  make build         - Build the main server"
	@echo "  make build-lambda  - Build the Lambda function ($(LAMBDA_BINARY))"
	@echo "  make build-all     - Build both server and Lambda"
	@echo "  make deploy-lambda - Deploy Lambda to AWS"
	@echo "  make run           - Run the local server"
	@echo "  make test          - Run tests"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make fix-lambda-imports - Fix lambda.rs import issues"
	@echo "  make invoke-lambda - Invoke Lambda function locally"
	@echo "  make watch-lambda  - Watch for changes and rebuild Lambda"

# Environment check
check-env:
ifndef LAMBDA_ROLE_ARN
	$(error LAMBDA_ROLE_ARN is not set. Please set it using: export LAMBDA_ROLE_ARN=arn:aws:iam::ACCOUNT_ID:role/ROLE_NAME)
endif
