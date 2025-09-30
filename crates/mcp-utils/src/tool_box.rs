use crate::tool::CustomTool;

#[macro_export]
macro_rules! setup_tools {
    ($visibility:vis $enum_name:ident, [$($tool_kind:ident ( $tool:ident ) ),* $(,)?]) => {
        $visibility struct $enum_name {
            inner: __tool_setup::InnerTools,
        }

        mod __tool_setup {
            use super::*;

            rust_mcp_sdk::tool_box!(InnerTools, [$($tool),*]);
        }

        impl $crate::server_prelude::ToolBox for $enum_name {
            fn get_tool(&self) -> $crate::tool_prelude::CustomTool {
                match &self.inner {
                    $(
                        __tool_setup::InnerTools::$tool(tool_value) => $crate::tool_prelude::CustomTool::$tool_kind(tool_value),
                    )*
                }
            }

            fn get_tools() -> Vec<rust_mcp_sdk::schema::Tool> {
                __tool_setup::InnerTools::tools()
            }
        }

        impl TryFrom<rust_mcp_sdk::schema::CallToolRequestParams> for $enum_name {
            type Error = rust_mcp_sdk::schema::schema_utils::CallToolError;

            fn try_from(mut value: rust_mcp_sdk::schema::CallToolRequestParams) -> Result<Self, Self::Error> {
                value.arguments.get_or_insert_default();
                Ok(Self {
                    inner: __tool_setup::InnerTools::try_from(value)?,
                })
            }
        }
    };
    ($enum_name:ident, [$($tool_kind:ident ( $tool:ident ) ),* $(,)?]) => {
        setup_tools!(pub(crate) $enum_name, [$($tool_kind ( $tool ) ),*]);
    };
}
pub use setup_tools;

pub trait ToolBox {
    fn get_tool(&'_ self) -> CustomTool<'_>;

    fn get_tools() -> Vec<rust_mcp_sdk::schema::Tool>;
}
