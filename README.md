# Black_Sniffer

This repository contains a simple command-line tool written in Rust for scanning open ports on a specified target IP address. The tool leverages asynchronous programming using `tokio` and the `clap` library for command-line argument parsing.

## Features

- **Target IP Address**: Scan a specified IP address.
- **Port Range**: Define a range of ports to scan by specifying start and end port numbers.
- **Connection Timeout**: Set a timeout duration for each connection attempt to determine if a port is open.

## Requirements

- Rust (Nightly version recommended)
- Cargo (Rust package manager)
- Tokio (Asynchronous runtime)
- Clap (Command-line argument parser)

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/port-sniffer.git
   cd port-sniffer
   ```
   
2. **Build the project**:
   ```bash
   cargo build --release
   ```
   
3. **Run the binary**:
   ```bash
   ./target/release/port-sniffer --help
   ```
   
## Usage

To run the port sniffer, use the following command:

```bash
./port-sniffer [OPTIONS]
