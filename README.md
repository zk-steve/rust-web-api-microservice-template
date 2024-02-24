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

Usage: cli [OPTIONS] [COMMAND]

Commands:
  config  Print config
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config-path <CONFIG_PATH>  Config file [default: config/default.toml]
  -v, --version                    Print version
  -h, --help                       Print help
```

### Example

- Multiple config locations

```shell
./cli -c ./config/* -c deploy/local/custom.toml
```

- Pipe the output with [bunyan](https://github.com/trentm/node-bunyan)

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

| ENV                                                                    | DEFAULT VALUE | NOTE      |
|------------------------------------------------------------------------|---------------|-----------|
| [RUST_LOG](https://docs.rs/env_logger/latest/env_logger/) > LOG__LEVEL | "INFO"        | Log level |
| SERVER__URL                                                            |               |           |
| SERVER__PORT                                                           |               |           |
| SERVICE_NAME                                                           |               |           |
| EXPORTER_ENDPOINT                                                      |               |           |
| DB__PG__URL                                                            |               |           |
| DB__PG__MAX_SIZE                                                       |               |           |

Make sure to set these environment variables according to your needs before running the server.

## GitHub Flow CI Configuration

1. **Set Docker Hub Secrets:**
   - Go to repository Settings > Secrets.
   - Add `DOCKER_USERNAME` and `DOCKERHUB_TOKEN`.

2. **Enable Dependabot Alerts:**
   - In repository Insights, enable "Dependabot alerts" and "Security & Analysis."

## Checklist

### Basic Functionalities

Ensure comprehension and implementation of concepts outlined in the book with attention to detail. Key considerations
include:

1. [x] Incorporating descriptive comments to enhance code readability.
2. [x] Implementing tracing mechanisms for effective debugging.
3. [x] Writing comprehensive test cases to validate functionality.
    1. [x] Using https://testcontainers.com for integration tests.
4. [x] Utilizing version control with [Git](https://git-scm.com/) for code management.
5. [x] Structuring code in a logical and maintainable manner.
6. [x] Containerizing the application using [Docker](https://www.docker.com/) for portability and scalability.

### Advanced Functionalities

Demonstrate proficiency in advanced development practices including:

1. [x] CLI Interface.
    1. [x] Embed Git Info, Config Tool.
2. [x] Load Configuration from a File.
3. [x] Multiple Implementations.
4. [x] Advanced Tracing.
5. [x] CI/CD.
    1. [ ] Migrate DB tool/image.
    2. [x] Publish binary artifacts in [Github](https://github.com/).
    3. [x] Push Docker images.
    4. [x] 
       Deploy [Dependabot](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuring-dependabot-version-updates)
6. [x] Docker Image Optimization.
7. [x] Load test using [K6](https://k6.io/).
    1. [x] Use [Flamegraph](https://github.com/flamegraph-rs/flamegraph) for profiling.
    2. [ ] [Better UI](https://medium.com/swlh/beautiful-load-testing-with-k6-and-docker-compose-4454edb3a2e3).
8. [ ] Comprehensive DB query filter for list().
9. [ ] Optimize release binary performance.
10. [ ] Docs on how to use this repo, the design behind the scene.
    Feel free to explore and expand upon these functionalities as needed for your project. Happy coding!

## Load Testing and Profiling

For load testing and profiling your Rust API server, refer to
the [Load Testing and Profiling with K6 and Flamegraph](./load-tests/README.md) guide. This document provides
detailed instructions on using K6 and Flamegraph for load testing and profiling purposes.