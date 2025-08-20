<div align="center">

[![checks](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml/badge.svg)](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml)
[![version](https://img.shields.io/crates/v/mcp-utils)](https://crates.io/crates/mcp-utils)
[![license](https://img.shields.io/crates/l/mcp-utils)](../../LICENSE.txt)
[![GitHub top language](https://img.shields.io/github/languages/top/seaofvoices/rust-mcp-utils)](https://www.rust-lang.org/)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/seaofvoices)

</div>

# mcp-utils

This project extends the [`rust-mcp-sdk`](https://crates.io/crates/rust-mcp-sdk) with additional conveniences for building MCP servers in Rust with a **defined set of tools**.

Derive one of the tool traits from `mcp-utils` to get:

- quick server setup: list your tools in `setup_tools!` and automatically get a server that can list your tools and handle tool calls
- switch between `async` or not: no need to adjust anything beside the tool trait you implement
- flexible output: return `Result` objects, plain strings, or anything that implements `Serialize`

For complete documentation and examples, see the main [project README](https://github.com/seaofvoices/rust-mcp-utils/blob/main/README.md).

## License

This project is available under the MIT license. See [LICENSE.txt](../../LICENSE.txt) for details.
