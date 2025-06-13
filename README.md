# NEPSE Auth Token Service

A lightweight Rust-based microservice that provides NEPSE (Nepal Stock Exchange) authentication token functionality. This service exposes a simple HTTP endpoint to generate and retrieve access tokens for NEPSE API interactions.

## Features

- RESTful API endpoint for token generation
- Built with Rust and Axum framework
- WASM module support
- Comprehensive logging and tracing
- Error handling with detailed responses

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/theArjun/nepse-auth-token.git
cd nepse-auth-token
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8888`

## API Usage

### Get Access Token

```http
GET /
```

Response:
```json
{
    "access_token": "your_access_token"
}
```

## Error Handling

The service returns appropriate HTTP status codes and error messages in case of failures:

```json
{
    "error": "Error message description"
}
```