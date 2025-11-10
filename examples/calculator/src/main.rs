mod sum_tool;

use sum_tool::SumTool;

use mcp_utils::server_prelude::*;

setup_tools!(pub Tools, [
    structured(SumTool),
]);

fn main() -> Result<(), String> {
    let server = mcp_cli_builder::ServerBuilder::new()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_title("Calculator MCP Server")
        .with_instructions(concat!(
            "A simple calculator server that provides basic arithmetic operations.\n\n",
            "All tools return structured results with either a calculated value or an error message."
        ));

    mcp_cli_builder::run::<Tools>(server)
}
