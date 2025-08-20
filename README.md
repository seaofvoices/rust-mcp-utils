<div align="center">

[![checks](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml/badge.svg)](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml)
[![version](https://img.shields.io/crates/v/mcp-utils)](https://crates.io/crates/mcp-utils)
[![license](https://img.shields.io/crates/l/mcp-utils)](LICENSE.txt)
[![GitHub top language](https://img.shields.io/github/languages/top/seaofvoices/rust-mcp-utils)](https://www.rust-lang.org/)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/seaofvoices)

</div>

# Rust MCP Utils

This project extends the [`rust-mcp-sdk`](https://crates.io/crates/rust-mcp-sdk) with additional conveniences for building MCP servers in Rust with a **defined set of tools**. It provides two crates:

- **`mcp-utils`** - Higher-level traits and abstractions that simplify tool definition and server setup beyond the base SDK
- **`mcp-cli-builder`** - Command-line interface generation for MCP servers built with `mcp-utils`

Both crates are built on top of `rust-mcp-sdk` and provide ergonomic wrappers around its core functionality.

Derive one of the tool traits from mcp-utils to get:

- quick server setup: list your tools in `setup_tools!` and automatically get a server that can list your tools and handle tool calls
- switch between `async` or not: no need to adjust anything beside the tool trait you implement
- flexible output: return `Result` objects, plain strings, or anything that implements `Serialize`

## Installation

Add the crates to your dependencies:

```bash
cargo add mcp-utils mcp-cli-builder
```

## Defining Tools

The `mcp-utils` crate provides several tool trait options you can implement:

- **`TextTool`** – Returns plain text responses (synchronous)
- **`StructuredTool`** – Returns structured JSON data (synchronous)
- **`AsyncTextTool`** – Returns plain text responses (asynchronous)
- **`AsyncStructuredTool`** – Returns structured JSON data (asynchronous)

Create tools by implementing one of these traits with the `#[mcp_tool]` attribute:

```rust
use mcp_utils::tool_prelude::*;

#[mcp_tool(
    name = "example_tool",
    description = "An example tool for demonstration",
    title = "Example Tool",
    idempotent_hint = true,
    read_only_hint = true,
    destructive_hint = false,
    open_world_hint = false,
)]
#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct ExampleTool {
    /// A message to process
    pub message: String,
}

impl TextTool for ExampleTool {
    type Output = String;

    fn call(&self) -> Self::Output {
        format!("Processed: {}", self.message)
    }
}
```

The attribute macro `mcp_tool` is re-exported from the `rust-mcp-sdk` crate. You can find the available options to use in its [documentation](https://docs.rs/rust-mcp-sdk/latest/rust_mcp_sdk/macros/attr.mcp_tool.html).

## Aggregating Tools

Use the `setup_tools!` macro to create a tool collection. Map each tool to its kind like in the following example:

```rust
use mcp_utils::server_prelude::*;

setup_tools!(pub MyTools, [
    text(SimpleGreeter), // for TextTool
    structured(TestTool), // for StructuredTool
    async_text(FileReader), // for AsyncTextTool
    async_structured(DataProcessor), // for AsyncStructuredTool
]);
```

This will generate a set of tools named `MyTools` that you can pass to the CLI builder to initialize the MCP server.

## Command Line Builder

Generate a command-line interface that handles the MCP server startup. This will build a command line parser using [clap](https://docs.rs/clap/latest/clap/) with:

- options to start the server in stdio mode or with server-sent events (with `--host` and `--port`)
- a clear `help` command which includes the available tools.
- an option to change the default request timeout (in [humantime](https://docs.rs/humantime/latest/humantime/) format)

```rust
use mcp_utils::server_prelude::*;

// Aggregate your tools together
setup_tools!(pub MyTools, [
    text(ExampleTool),
    // ...
]);

fn main() -> Result<(), String> {
    let server = ServerBuilder::new()
        .with_name(env!("CARGO_PKG_NAME")) // uses the name from Cargo.toml
        .with_version(env!("CARGO_PKG_VERSION")) // uses the version from Cargo.toml
        .with_title("My MCP Server")
        .with_instructions("A demonstration MCP server");

    mcp_cli_builder::run::<MyTools>(server)
}
```

## License

This project is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.
