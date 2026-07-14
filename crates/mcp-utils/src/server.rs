use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use rust_mcp_actix::{ActixServerOptions, create_actix_server};
use rust_mcp_sdk::{
    McpServer, StdioTransport, ToMcpServerHandler, TransportOptions,
    error::McpSdkError,
    mcp_server::{McpServerOptions, ServerHandler, server_runtime::create_server},
    schema::{
        CallToolRequestParams, CallToolResult, Implementation, InitializeResult,
        LATEST_PROTOCOL_VERSION, ListToolsResult, PaginatedRequestParams, RpcError,
        ServerCapabilities, ServerCapabilitiesTools, schema_utils::CallToolError,
    },
};

use crate::{server_config::ServerConfig, tool_box::ToolBox};

#[derive(Debug, Clone, Default)]
pub struct ServerBuilder {
    config: ServerConfig,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.config.name = name.into();
        self
    }

    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.config.instructions = instructions.into();
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.config.version = version.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.config.name = name.into();
    }

    pub fn set_instructions(&mut self, instructions: impl Into<String>) {
        self.config.instructions = instructions.into();
    }

    pub fn set_version(&mut self, version: impl Into<String>) {
        self.config.version = version.into();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.config.title = title.into();
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.config.timeout = timeout;
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn title(&self) -> &str {
        &self.config.title
    }

    pub fn version(&self) -> &str {
        &self.config.version
    }

    pub fn instructions(&self) -> &str {
        &self.config.instructions
    }

    pub async fn start_stdio<T>(self) -> Result<(), McpSdkError>
    where
        T: ToolBox + TryFrom<CallToolRequestParams, Error = CallToolError> + Send + Sync + 'static,
    {
        let transport_options = TransportOptions {
            timeout: self.config.timeout,
            ..Default::default()
        };

        create_server(McpServerOptions {
            server_details: self.get_server_details::<T>(),
            transport: StdioTransport::new(transport_options)?,
            handler: Handler::<T>::new().to_mcp_server_handler(),
            task_store: None,
            client_task_store: None,
            message_observer: None,
        })
        .start()
        .await
    }

    pub async fn start_server<T>(
        self,
        host: impl Into<String>,
        port: u16,
    ) -> Result<(), McpSdkError>
    where
        T: ToolBox + TryFrom<CallToolRequestParams, Error = CallToolError> + Send + Sync + 'static,
    {
        let transport_options = TransportOptions {
            timeout: self.config.timeout,
            ..Default::default()
        };

        create_actix_server(
            self.get_server_details::<T>(),
            Handler::<T>::new().to_mcp_server_handler(),
            ActixServerOptions {
                host: Some(host.into())
                    .filter(|host| !host.is_empty())
                    .unwrap_or_else(|| "127.0.0.1".to_string()),
                port,
                transport_options: Arc::new(transport_options),
                ..Default::default()
            },
        )
        .start()
        .await
    }

    fn get_server_details<T>(self) -> InitializeResult
    where
        T: ToolBox,
    {
        InitializeResult {
            server_info: Implementation {
                name: self.config.name,
                version: self.config.version,
                title: Some(self.config.title).filter(|title| !title.is_empty()),
                description: Some(self.config.description)
                    .filter(|description| !description.is_empty()),
                website_url: None,
                icons: Default::default(),
            },
            capabilities: ServerCapabilities {
                tools: if T::get_tools().is_empty() {
                    None
                } else {
                    Some(ServerCapabilitiesTools { list_changed: None })
                },
                ..Default::default()
            },
            meta: None,
            instructions: Some(self.config.instructions),
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        }
    }
}

struct Handler<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Handler<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
#[allow(unused)]
impl<T> ServerHandler for Handler<T>
where
    T: ToolBox + TryFrom<CallToolRequestParams, Error = CallToolError> + Send + Sync + 'static,
{
    async fn handle_list_tools_request(
        &self,
        params: Option<PaginatedRequestParams>,
        runtime: Arc<dyn McpServer>,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: T::get_tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        runtime: Arc<dyn McpServer>,
    ) -> Result<CallToolResult, CallToolError> {
        let custom_tool = T::try_from(params).map_err(CallToolError::new)?;

        custom_tool.get_tool().call().await
    }
}
