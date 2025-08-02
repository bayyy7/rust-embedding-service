# Rust gRPC Embedding Service with FastEmbed

A high-performance gRPC-based embedding service built in Rust using FastEmbed and the `sentence-transformers/paraphrase-MiniLM-L12-v2` model. You can change to others model by modify the code on `main.py`:

```rust
InitOptions::new(EmbeddingModel::ParaphraseMLMiniLML12V2Q)
```

# Features

- **Fast & Lightweight**: Built in Rust for optimal performance and low resource consumption
- **gRPC API**: Efficient binary protocol for client-server communication
- **Flexible Input**: Supports both single text and batch processing
- **ONNX Model**: Uses FastEmbed with ONNX runtime for fast inference
- **Configurable**: Environment-based configuration for easy deployment

# Quick Start

## 1. Build from Code

### Prerequisites

- Rust (latest stable version)
- Protocol Buffers compiler (`protoc`)

1. Create a `.env` with values:

```bash
APP_SERVER_ADDRESS=0.0.0.0:6010
```

2. Build the project:

```bash
cargo build --release
```

3. Start the server:

```bash
cargo run --bin server
```

The server will start on `127.0.0.1:6010` by default and download the embedding model on first run.

## 2. Build from DockerFile

1. Create a `.env` with values:

```bash
APP_SERVER_ADDRESS=0.0.0.0:6010
```

2. Change the `<your_image_name>` with your desired name and run it:

```bash
docker build -t <your_image_name>:latest .
```

## 3. Running from Docker Compose (Recommended)

```bash
docker compose up --build -d
```

## API Reference

### gRPC Service: `EmbeddingService`

#### Method: `GetEmbeddings`

**Request** (`EmbeddingRequest`):

```protobuf
message EmbeddingRequest {
  oneof input {
    string single_text = 1;        // Single text input
    TextBatch batch_texts = 2;     // Batch text input
  }
}

message TextBatch {
  repeated string texts = 1;
}
```

**Response** (`EmbeddingResponse`):

```protobuf
message EmbeddingResponse {
  repeated Vector vectors = 1;
}

message Vector {
  repeated float values = 1;
}
```

## Performance Notes

- The service loads the model once at startup for optimal performance
- FastEmbed uses ONNX runtime for efficient inference
- Batch processing is recommended for multiple texts to reduce overhead
- Model files are cached locally after first download

## Created by:

### Rizky Indrabayu
