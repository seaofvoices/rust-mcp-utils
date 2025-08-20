//! # mcp-cli-builder
//!
//! Command-line interface generation for MCP servers built with [`mcp-utils`](https://docs.rs/mcp-utils/latest/mcp_utils/index.html).
//!
//! For more information, see the **repository's [documentation](https://github.com/seaofvoices/rust-mcp-utils)**.
//!
//! This crate provides utilities to automatically generate command-line interfaces for MCP (Model Context Protocol) servers using
//! [`clap`](https://docs.rs/clap/latest/clap/).
//!
//! ## Features
//!
//! - **Automatic CLI generation**: Creates a complete command-line interface from your [`ServerBuilder`] configuration
//! - **Dual server modes**: Supports both stdio (default) and HTTP server modes with `--host`/`--port` options
//! - **Rich help output**: Automatically generates help text with tool descriptions and usage instructions
//! - **Timeout configuration**: Built-in support for request timeouts using [`humantime`](https://docs.rs/humantime/latest/humantime/) formats
//! - **Zero configuration**: Works out of the box with any [`ToolBox`] implementation

use std::{env, ffi::OsString};

use clap::{Arg, Command};
pub use mcp_utils::server_prelude::ServerBuilder;
use mcp_utils::server_prelude::ToolBox;
use rust_mcp_sdk::{
    error::McpSdkError,
    schema::{CallToolRequestParams, schema_utils::CallToolError},
};

const DEFAULT_PORT: u16 = 8080;

const ARG_TIMEOUT: &str = "timeout";
const ARG_HOST: &str = "host";
const ARG_PORT: &str = "port";

/// Runs an MCP server with automatically generated command-line interface.
///
/// This function creates a complete CLI application from a [`ServerBuilder`] configuration
/// and tool collection. It handles argument parsing, generates help text with tool descriptions,
/// and starts the server in either stdio or HTTP mode based on the provided arguments.
///
/// The function automatically parses command-line arguments from [`std::env::args_os()`]
/// and configures the server accordingly. If parsing fails, it displays help and exits.
/// Runtime errors are returned as formatted strings.
///
/// # Type Parameters
///
/// * `T`
///
///   A type implementing [`ToolBox`] that represents your collection of MCP tools.
///   This is generated using the `setup_tools!` macro from [`mcp-utils`](https://docs.rs/mcp-utils/latest/mcp_utils/index.html).
///
/// # Server Behavior
///
/// - When called **without** `--host` or `--port` the server starts in stdio mode
/// - When called **with** `--host` and/or `--port` the server starts an HTTP server with Server-Sent Events
///
/// # Examples
///
/// ```rust,no_run
/// use mcp_cli_builder::{run, ServerBuilder};
/// use mcp_utils::{tool_prelude::*, server_prelude::*};
///
/// # #[mcp_tool(name = "example", description = "An example tool")]
/// # #[derive(Debug, JsonSchema, Serialize, Deserialize)]
/// # pub struct ExampleTool { pub message: String }
/// # impl TextTool for ExampleTool {
/// #     type Output = String;
/// #     fn call(&self) -> Self::Output { self.message.clone() }
/// # }
/// setup_tools!(pub MyTools, [
///     text(ExampleTool),
/// ]);
///
/// fn main() -> Result<(), String> {
///     let builder = ServerBuilder::new()
///         .with_name(env!("CARGO_PKG_NAME")) // uses the name from Cargo.toml
///         .with_version(env!("CARGO_PKG_VERSION")) // uses the version from Cargo.toml
///         .with_title("My MCP Server")
///         .with_instructions("Demonstrates MCP server functionality");
///
///     run::<MyTools>(builder)
/// }
/// ```
pub fn run<T>(builder: ServerBuilder) -> Result<(), String>
where
    T: ToolBox + TryFrom<CallToolRequestParams, Error = CallToolError> + Send + Sync + 'static,
{
    match inner_run::<T, _>(builder, env::args_os()) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(start_error)) => {
            eprintln!(
                "{}",
                start_error
                    .rpc_error_message()
                    .unwrap_or(&start_error.to_string())
            );
            Err(start_error.to_string())
        }
        Err(clap_err) => clap_err.exit(),
    }
}

fn inner_run<T, IntoArg>(
    mut builder: ServerBuilder,
    args: impl IntoIterator<Item = IntoArg>,
) -> Result<Result<(), McpSdkError>, clap::Error>
where
    T: ToolBox + TryFrom<CallToolRequestParams, Error = CallToolError> + Send + Sync + 'static,
    IntoArg: Into<OsString> + Clone,
{
    let bold = clap::builder::styling::Style::new().bold();
    let underlined = clap::builder::styling::Style::new().underline();
    let dimmed = clap::builder::styling::Style::new().dimmed();

    let tools = T::get_tools();
    let mut tool_names: Vec<_> = tools
        .iter()
        .enumerate()
        .map(|(i, tool)| {
            if let Some(description) = tool.description.as_ref() {
                format!(
                    "{}. {underlined}{}{underlined:#}\n    {}",
                    i + 1,
                    tool.title.as_ref().unwrap_or(&tool.name).as_str(),
                    description
                )
            } else {
                format!(
                    "{}. {underlined}{}{underlined:#}: {dimmed}no description available{dimmed:#}",
                    i + 1,
                    tool.title.as_ref().unwrap_or(&tool.name).as_str(),
                )
            }
        })
        .collect();
    tool_names.sort();

    let matches = Command::new(builder.name().to_owned())
        .about(format!(
            r#"{underlined}{}{underlined:#}

Start the MCP server in stdio mode by running the command:
  {bold}{}{bold:#}

To use SSE (Server-Sent Events), pass the --host and/or the --port options
  {bold}{} --port 8080{bold:#}
"#,
            builder.title(),
            builder.name(),
            builder.name(),
        ))
        .version(builder.version().to_owned())
        .after_long_help(format!(
            "MCP server: {}\n\n{bold}Instructions:{bold:#}\n{}\n\n{bold}Tools:{bold:#}\n{}",
            builder.title(),
            builder.instructions(),
            tool_names.join("\n")
        ))
        .arg(
            Arg::new(ARG_TIMEOUT)
                .help("Timeout for requests made  (in humantime format, see <https://docs.rs/humantime/latest/humantime/>)")
                .default_value("60s")
                .long("timeout")
                .value_parser(clap::value_parser!(humantime::Duration)),
        )
        .arg(
            Arg::new(ARG_HOST)
                .help("Host to bind the server to")
                .long("host")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new(ARG_PORT)
                .help("Port to bind the server to")
                .long("port")
                .short('p')
                .value_parser(clap::value_parser!(u16)),
        )
        .try_get_matches_from(args)?;

    let timeout = matches
        .get_one::<humantime::Duration>(ARG_TIMEOUT)
        .cloned()
        .map(Into::into)
        .unwrap_or_else(|| std::time::Duration::from_secs(60));

    builder.set_timeout(timeout);

    let host = matches.get_one::<String>(ARG_HOST).cloned();
    let port = matches.get_one::<u16>(ARG_PORT).cloned();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            Ok(match (host, port) {
                (None, None) => builder.start_stdio::<T>().await,
                (host, port) => {
                    builder
                        .start_server::<T>(
                            host.as_deref().unwrap_or("127.0.0.1"),
                            port.unwrap_or(DEFAULT_PORT),
                        )
                        .await
                }
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcp_utils::server_prelude::setup_tools;
    use mcp_utils::tool_prelude::*;

    #[mcp_tool(
        name = "test_tool",
        description = "A test tool for demonstration",
        title = "Test Tool"
    )]
    #[derive(Debug, JsonSchema, Serialize, Deserialize)]
    pub struct TestTool {
        /// A message to process
        pub message: String,
    }

    impl StructuredTool for TestTool {
        type Output = String;

        fn call(&self) -> Self::Output {
            format!("Processed: {}", self.message)
        }
    }

    #[mcp_tool(name = "another_tool", description = "A tool that doubles a number")]
    #[derive(Debug, JsonSchema, Serialize, Deserialize)]
    pub struct AnotherTool {
        /// A value to double
        pub value: i32,
    }

    impl StructuredTool for AnotherTool {
        type Output = i32;

        fn call(&self) -> Self::Output {
            self.value * 2
        }
    }

    // Use the setup_tools macro to create a proper ToolBox
    setup_tools!(pub TestTools, [
        structured(TestTool),
        structured(AnotherTool),
    ]);

    fn get_builder() -> ServerBuilder {
        ServerBuilder::new()
            .with_name("test-server")
            .with_title("Test MCP Server")
            .with_version("1.0.0")
            .with_instructions("This is a test server for demonstration purposes")
    }

    #[test]
    fn test_help_command_snapshot() {
        let builder = get_builder();

        let help_output = match inner_run::<TestTools, _>(builder, ["test-server", "--help"]) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("Expected help error, but inner_run succeeded"),
        };

        insta::assert_snapshot!("help_output", help_output);
    }

    #[test]
    fn test_short_help_command_snapshot() {
        let builder = get_builder();

        let err = match inner_run::<TestTools, _>(builder, ["test-server", "-h"]) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("Expected help error, but inner_run succeeded"),
        };

        insta::assert_snapshot!("help_short_output", err);
    }

    #[test]
    fn test_version_command_snapshot() {
        let builder = get_builder();

        let err = match inner_run::<TestTools, _>(builder, ["test-server", "--version"]) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("Expected help error, but inner_run succeeded"),
        };

        insta::assert_snapshot!("version_output", err);
    }
}
