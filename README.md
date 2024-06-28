# Cricket - Lightweight Server Monitoring Agent

Cricket is a lightweight server monitoring agent designed to collect and report server metrics, custom metrics, events, and service checks. The collected data is sent to a backend API for monitoring and visualization.

## Features

- Collects basic metrics like CPU load and aliveness.
- Supports custom metrics via `statsd`.
- Collects events and service checks.
- Uses RSA key pairs for secure communication.
- Configurable via command-line arguments or a configuration file.
- Asynchronous operation using `tokio` for efficient resource usage.


## Getting Started

### Prerequisites

- Rust (latest stable version)
- OpenSSL (for RSA key generation and signing)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/cricket.git
    cd cricket
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

### Configuration

Cricket can be configured using a configuration file (`config.toml`) or command-line arguments.

Example `config.toml`:
```toml
api_url = "https://localhost:8000"
api_key = "your_api_key"
interval = 5
submission_interval = 15
key_path = "./keys"
```

## Running Cricket
You can run Cricket with the following command:

```sh
cargo run --release -- --api_url "https://api.example.com" --api_key "your_api_key" --interval 5 --submission_interval 15 --key_path "./keys"
```

## Command-Line Arguments
  - api_url: API URL
  - api_key: API Key for initial bootstrap
  - interval: Interval in seconds between metric collections (default: 5)
  - submission_interval: Interval in seconds for sending metrics (default: 15)
  - key_path: Path to the key pair


## Running a Test HTTP Server

To test the API functionality, you can run a simple Python HTTP server that responds with status 200 to every request:

```shell
python simple_server.py
```
