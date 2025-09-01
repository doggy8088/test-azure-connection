# AI Coding Guidelines for test-azure-connection

## Project Overview
This is a minimal Rust application for testing Azure OpenAI API connectivity, specifically designed to evaluate how musl-based compilation affects network connections. The project focuses on static linking and cross-compilation for deployment in constrained environments.

## Key Dependencies
- **reqwest** (0.11): HTTP client with `rustls-tls` feature for TLS support without OpenSSL
- **tokio** (1.x): Async runtime with `full` features for comprehensive async support
- **serde_json** (1.0): JSON serialization/deserialization

## Build and Runtime Workflow
- **Target**: Always build for `x86_64-unknown-linux-musl` to ensure static linking
- **Environment Variables**:
  - `AZURE_OPENAI_API_KEY`: Your Azure OpenAI API key
  - `AZURE_BASE_URL`: Base URL like `https://your-resource.openai.azure.com/openai`
- **Build Command**: `cargo build --target x86_64-unknown-linux-musl`
- **Run Command**: `./target/x86_64-unknown-linux-musl/debug/test-azure-connection`

## Code Patterns
- **Error Handling**: Use `?` operator for propagating errors in async contexts
- **Async Main**: Always use `#[tokio::main]` for the main function
- **HTTP Requests**:
  - Use `reqwest::Client::new()` for client creation
  - Set `api-key` header for Azure authentication
  - Use `serde_json::json!` macro for request body construction
- **API Endpoint**: Target `/deployments/{model}/chat/completions?api-version=2025-01-01-preview`

## Architecture Decisions
- **Musl Compilation**: Ensures fully static binaries for deployment without external dependencies
- **Rustls TLS**: Avoids OpenSSL for better compatibility in musl environments
- **Minimal Dependencies**: Only essential crates to keep binary size small and build fast

## Testing Approach
- **Integration Testing**: Test actual API calls with real credentials
- **Output Validation**: Check HTTP status codes and response structure
- **Network Debugging**: Use `println!` for status and raw response inspection

## File Structure
- `src/main.rs`: Single-file application containing all logic
- `Cargo.toml`: Minimal dependency configuration
- `README.md`: Build and usage instructions in Chinese
