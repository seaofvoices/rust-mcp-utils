//! # mcp-utils
//!
//! Abstractions that extend [`rust-mcp-sdk`](https://docs.rs/rust-mcp-sdk/latest/rust_mcp_sdk/index.html) for simplified MCP tool definition and server setup.
//!
//! For more information, see the **repository's [documentation](https://github.com/seaofvoices/rust-mcp-utils)**.
//!
//! This crate provides higher-level traits and abstractions that simplify tool definition and server setup beyond the base SDK.
//! It offers ergonomic wrappers around the core MCP functionality with automatic serialization and flexible output handling.
//!
//! ## Tool Traits
//!
//! The crate provides several tool trait options you can implement:
//!
//! - [`tool::TextTool`] – Returns plain text responses (synchronous)
//! - [`tool::AsyncTextTool`] – Returns plain text responses (asynchronous)
//! - [`tool::StructuredTool`] – Returns structured JSON data (synchronous)
//! - [`tool::AsyncStructuredTool`] – Returns structured JSON data (asynchronous)
//!
//! All traits provide flexible output handling. Return [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
//! objects, plain strings, or anything that implements [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html).
//!
//! ## Quick Start
//!
//! ### 1. Define a Tool
//!
//! ```rust
//! use mcp_utils::tool_prelude::*;
//!
//! #[mcp_tool(
//!     name = "example_tool",
//!     description = "An example tool for demonstration",
//!     title = "Example Tool",
//!     idempotent_hint = true,
//!     read_only_hint = true,
//!     destructive_hint = false,
//!     open_world_hint = false,
//! )]
//! #[derive(Debug, JsonSchema, Serialize, Deserialize)]
//! pub struct ExampleTool {
//!     /// A message to process
//!     pub message: String,
//! }
//!
//! impl TextTool for ExampleTool {
//!     type Output = String;
//!
//!     fn call(&self) -> Self::Output {
//!         format!("Processed: {}", self.message)
//!     }
//! }
//! ```
//!
//! ### 2. Aggregate Tools
//!
//! After defining your tools, use the [`setup_tools!`] macro to create a tool collection:
//!
//! ```rust
//! use mcp_utils::tool_prelude::*;
//! # use mcp_utils::server_prelude::*;
//! # #[mcp_tool(name = "example_tool", description = "An example tool for demonstration")]
//! # #[derive(Debug, JsonSchema, Serialize, Deserialize)]
//! # pub struct ExampleTool {
//! #     pub name: String,
//! # }
//! # impl TextTool for ExampleTool {
//! #     type Output = String;
//! #     fn call(&self) -> Self::Output {
//! #         format!("Hello, {}!", self.name)
//! #     }
//! # }
//!
//! setup_tools!(pub MyTools, [
//!     text(ExampleTool),
//! ]);
//!
//! # fn main () {}
//! ```
//!
//! ## Prelude Modules
//!
//! This crate provides two prelude modules for convenient imports:
//!
//! - [`tool_prelude`] - Everything needed for defining tools
//! - [`server_prelude`] - Everything needed for server setup and tool aggregation

mod server;
mod server_config;
mod tool;
mod tool_box;

pub mod tool_prelude {
    //! Everything needed for defining MCP tools.
    //!
    //! This module re-exports the tool traits, error types, and necessary macros
    //! from both this crate and `rust-mcp-sdk`.

    pub use super::tool::{
        AsyncStructuredTool, AsyncTextTool, CustomTool, StructuredTool, TextTool, ToolError,
    };
    pub use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
    pub use serde::{Deserialize, Serialize};
}

pub mod server_prelude {
    //! Everything needed for server setup and tool aggregation.
    //!
    //! This module provides the server builder, tool aggregation macro, and related types.

    pub use super::server::ServerBuilder;
    pub use super::tool_box::{ToolBox, setup_tools};
    pub use rust_mcp_sdk::mcp_server::ServerRuntime;
}
