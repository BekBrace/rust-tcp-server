# rust-tcp-server
This is creating a TCP Server using Rust  programming language 
# Rust TCP Server [YouTube Video Tutorial | Bek Brace]

A simple TCP server implemented in Rust for educational purposes. This project demonstrates the basics of networking in Rust and how to create a basic server that handles client connections.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Building](#building)
  - [Running](#running)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- Accepts incoming TCP connections.
- Handles client requests by sending a "Hello, Client!" response.
- Multithreading to handle multiple client connections simultaneously.

## Getting Started

To get a copy of this project up and running on your local machine, follow these steps:

### Prerequisites

- Rust programming language installed. You can download it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Building

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/BekBrace/rust-tcp-server.git

Navigate to the project directory:
cd rust-tcp-server

Build the project:
cargo build

Start the server:
cargo run

The server will start listening on 127.0.0.1:8080.

Usage
Connect to the server using a TCP client (e.g., Telnet or opening a web browser like demonstrated in the YT tutorial) to test the functionality.


