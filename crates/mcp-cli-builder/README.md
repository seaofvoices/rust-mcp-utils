<div align="center">

[![checks](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml/badge.svg)](https://github.com/seaofvoices/rust-mcp-utils/actions/workflows/test.yml)
[![version](https://img.shields.io/crates/v/mcp-cli-builder)](https://crates.io/crates/mcp-cli-builder)
[![license](https://img.shields.io/crates/l/mcp-cli-builder)](../../LICENSE.txt)
[![GitHub top language](https://img.shields.io/github/languages/top/seaofvoices/rust-mcp-utils)](https://www.rust-lang.org/)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/seaofvoices)

</div>

# mcp-cli-builder

Generate a command-line interface that handles the MCP server startup. This will build a command line parser using [clap](https://docs.rs/clap/latest/clap/) with:

- options to start the server in stdio mode or with server-sent events (with `--host` and `--port`)
- a clear `help` command which includes the available tools.
- an option to change the default request timeout (in [humantime](https://docs.rs/humantime/latest/humantime/) format)

For complete documentation and examples, see the main [project README](https://github.com/seaofvoices/rust-mcp-utils/blob/main/README.md).

## License

This project is available under the MIT license. See [LICENSE.txt](../../LICENSE.txt) for details.
