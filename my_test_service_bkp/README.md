# my_test_service

This is a Rust service that can be deployed both as a standalone server and as an AWS Lambda function.

## Project Structure

```
src/
├── common/         # Common utilities and types
├── process/        # Core processing logic
├── routes/         # API route handlers
└── services/       # Entry points for server and lambda
```

## Running Locally

To run the server locally:

```bash
cargo run --bin server
```

The server will start on http://localhost:3000

## Building for Lambda

To build for AWS Lambda:

```bash
cargo build --release --bin lambda
```

## Development

1. Add your processing logic in the `process` module
2. Update the API routes in `routes`
3. Add any shared types or utilities in `common`
4. Implement the specific handlers in `services/main.rs` and `services/lambda.rs`

## Testing

Run the tests with:

```bash
cargo test
``` 