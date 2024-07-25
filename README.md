# Description
Apigen is a modular tool for generating type safe APIs from [OpenAPI specs](https://swagger.io/specification/).

This package contains a CLI tool (a runner) that relies on separate plugins to generate code for different languages.

# Motivation

The OpenAPI spec is the de facto standard for describing RESTful APIs. I often find myself writing code in multiple languages that interacts with the same API. This tool aims to reduce the amount of boilerplate code that needs to be written by both clients and servers.

# Build
run `cargo build --release` to build the runner

# Usage
Start the executablew with the `--help` flag to see the available options.

```shell
apigen --help
```

# Plugins

* [apigen-RS](https://github.com/naskopw/apigen-rs) - Rust plugin
* [apigen-TS](https://github.com/naskopw/apigen-ts) - TypeScript plugin

# Plugin discovery
The runner can either load plugins via a filesystem path, or by searching the System PATH for executables with the name `apigen-<plugin-name>`. Run `apigen --help` for more information.

# The plugin protocol

Plugins are standalone executables that read the OpenAPI spec (as JSON) from stdin and write the generated code to stdout. This approach allows for plugins to be written in any language.


## Input
The OpenAPI spec is passed to the plugin as a JSON object. The plugin should read this object from stdin. The input is expected to be a valid [OpenAPI spec](https://swagger.io/specification/).

## Code generation
The generated code is then written to an output file by this runner.

## Error handling
In case of an error, the plugin should write the error message to stderr and exit with a non-zero status code.
