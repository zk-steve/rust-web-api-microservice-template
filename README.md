# RUST API SERVER

## Introduction

Welcome to the Rust API Server! This server provides a simple REST interface for your applications. This README will
guide you through setting up and running the server, as well as configuring its various options.

## How To Run

To get started, execute the following command in your terminal:

```shell
./cli --help
```

This will display the available options for running the server:

```
Simple REST server

Usage: cli [OPTIONS]

Options:
  -c, --config-file <CONFIG_FILE>  Config file [default: config/default.toml]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Example

- Multiple config locations

```shell
./cli -c ./config/* -c deploy/local/custom.toml
```

- Pipe the output with bunyan

```shell
cargo install bunyan
./cli -c ./config/* -c deploy/local/custom.toml | bunyan
```

## Configuration

### Order of apply

Configuration is applied in the following order: config files -> environment variables -> command-line arguments.

### Environment Variable Examples

The server can be configured using environment variables. Below is a table outlining the available configuration
options:

Hierarchical child config via env, separated by using `__`. Specify list values by using `,` separator

| ENV                  | DEFAULT VALUE | NOTE      |
|----------------------|---------------|-----------|
| RUST_LOG > LOG_LEVEL | "INFO"        | Log level |
| WEB_URL              |               |           |
| SERVICE_NAME         |               |           |
| EXPORTER_ENDPOINT    |               |           |
| PG__USERNAME         |               |           |
| PG__PASSWORD         |               |           |
| PG__HOST             |               |           |
| PG__PORT             |               |           |
| PG__DBNAME           |               |           |
| PG__MAX_SIZE         |               |           |

Make sure to set these environment variables according to your needs before running the server.

## Checklist

### Basic Functionalities

Ensure comprehension and implementation of concepts outlined in the book with attention to detail. Key considerations
include:

1. [x] Incorporating descriptive comments to enhance code readability.
2. [x] Implementing tracing mechanisms for effective debugging.
3. [ ] Writing comprehensive test cases to validate functionality.
4. [x] Utilizing version control with Git for code management.
5. [ ] Structuring code in a logical and maintainable manner.
6. [x] Containerizing the application using Docker for portability and scalability.

### Advanced Functionalities

Demonstrate proficiency in advanced development practices including:

1. [x] CLI Interface.
2. [x] Load Configuration from a File.
3. [ ] Multiple Implementations.
4. [x] Advanced Tracing.
5. [ ] CI/CD.
    1. [ ] Migrate DB tool/image.
6. [x] Docker Image Optimization.
7. [ ] Load test using K6s.

Feel free to explore and expand upon these functionalities as needed for your project. Happy coding!