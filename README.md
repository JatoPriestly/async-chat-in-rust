# async-chat

Async Chat in Rust

Overview

This project is an asynchronous chat server written in Rust. It demonstrates how to handle multiple client connections concurrently using asynchronous programming. The server supports functionalities such as joining chat groups, posting messages, and sending error messages back to clients.

Features

Asynchronous handling of client connections

Joining chat groups

Posting messages to groups

Error handling and reporting


Getting Started

Prerequisites

Ensure you have Rust installed. You can install it from rust-lang.org.

Installation

1. Clone the repository:

git clone https://github.com/JatoPriestly/async-chat-in-rust.git


2. Change to the project directory:

cd async-chat-in-rust


3. Build the project:

cargo build



Running the Server

To run the chat server, use the following command:

cargo run --bin server

Running the Client

To run the chat client, use the following command:

cargo run --bin client

Project Structure

- src/
  - bin/
    - server.rs       # Main function to start the chat server
    - client.rs       # Main function to start the chat client
  - group_table.rs    # Manages chat groups
  - connection.rs     # Handles client connections and communication
  - utils.rs          # Utility functions for sending and receiving JSON messages

Usage

1. Start the server.


2. Start one or more clients.


3. Clients can join groups and post messages which will be broadcast to all members of the group.



Example Commands

Join a group: Send a JSON message with the format:

{
  "Join": { "group_name": "example_group" }
}

Post a message: Send a JSON message with the format:

{
  "Post": {
    "group_name": "example_group",
    "message": "Hello, world!"
  }
}


Contributing

1. Fork the repository.


2. Create a new branch for your feature or bugfix.


3. Make your changes.


4. Submit a pull request.



License

This project is licensed under the MIT License. See the LICENSE file for details.

Contact

For any questions or issues, please open an issue on this repository or contact the maintainer.

