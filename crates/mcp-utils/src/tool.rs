use std::fmt;

use async_trait::async_trait;
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use serde::Serialize;

pub trait TextTool {
    type Output: IntoTextToolResult;

    fn call(&self) -> Self::Output;
}

#[async_trait]
pub trait AsyncTextTool {
    type Output: IntoTextToolResult;

    async fn call(&self) -> Self::Output;
}

pub trait IntoTextToolResult {
    fn result(self) -> Result<String, ToolError>;
}

impl IntoTextToolResult for String {
    fn result(self) -> Result<String, ToolError> {
        Ok(self)
    }
}

impl IntoTextToolResult for &String {
    fn result(self) -> Result<String, ToolError> {
        Ok(self.clone())
    }
}

impl IntoTextToolResult for &str {
    fn result(self) -> Result<String, ToolError> {
        Ok(self.to_string())
    }
}

impl<T, E> IntoTextToolResult for Result<T, E>
where
    T: Into<String>,
    E: Into<ToolError>,
{
    fn result(self) -> Result<String, ToolError> {
        self.map(|value| value.into()).map_err(|err| err.into())
    }
}

pub trait IntoStructuredToolResult {
    fn result(self) -> Result<serde_json::Value, ToolError>;
}

impl<T> IntoStructuredToolResult for T
where
    T: Serialize,
{
    fn result(self) -> Result<serde_json::Value, ToolError> {
        serde_json::to_value(self).map_err(|e| ToolError::from(e.to_string()))
    }
}

pub trait StructuredTool {
    type Output: IntoStructuredToolResult;

    fn call(&self) -> Self::Output;
}

#[async_trait]
pub trait AsyncStructuredTool {
    type Output: IntoStructuredToolResult;

    async fn call(&self) -> Self::Output;
}

#[derive(Debug)]
pub struct ToolError {
    display: String,
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl From<String> for ToolError {
    fn from(value: String) -> Self {
        Self { display: value }
    }
}

impl From<&str> for ToolError {
    fn from(value: &str) -> Self {
        Self {
            display: value.to_owned(),
        }
    }
}

impl From<&String> for ToolError {
    fn from(value: &String) -> Self {
        Self {
            display: value.clone(),
        }
    }
}

impl std::error::Error for ToolError {}

#[async_trait]
trait CustomTextTool {
    async fn call(&self) -> Result<CallToolResult, CallToolError>;
}

#[async_trait]
trait CustomStructuredTool {
    async fn call(&self) -> Result<CallToolResult, CallToolError>;
}

#[async_trait]
trait AsyncCustomTextTool {
    async fn call(&self) -> Result<CallToolResult, CallToolError>;
}

#[async_trait]
trait AsyncCustomStructuredTool {
    async fn call(&self) -> Result<CallToolResult, CallToolError>;
}

#[async_trait]
impl<T, O> CustomTextTool for T
where
    T: TextTool<Output = O> + Send + Sync,
    O: IntoTextToolResult,
{
    async fn call(&self) -> Result<CallToolResult, CallToolError> {
        let result = TextTool::call(self).result().map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::new(
            result, None, None,
        )]))
    }
}

#[async_trait]
impl<T, O> AsyncCustomTextTool for T
where
    T: AsyncTextTool<Output = O> + Send + Sync,
    O: IntoTextToolResult,
{
    async fn call(&self) -> Result<CallToolResult, CallToolError> {
        let result = AsyncTextTool::call(self)
            .await
            .result()
            .map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::new(
            result, None, None,
        )]))
    }
}

#[async_trait]
impl<T> CustomStructuredTool for T
where
    T: StructuredTool + Send + Sync,
    T::Output: IntoStructuredToolResult,
{
    async fn call(&self) -> Result<CallToolResult, CallToolError> {
        let value = StructuredTool::call(self)
            .result()
            .map_err(CallToolError::new)?;
        Ok(
            CallToolResult::text_content(vec![]).with_structured_content(match value {
                serde_json::Value::Object(map) => map,
                value => {
                    let mut map = serde_json::Map::new();
                    map.insert("result".to_string(), value);
                    map
                }
            }),
        )
    }
}

#[async_trait]
impl<T> AsyncCustomStructuredTool for T
where
    T: AsyncStructuredTool + Send + Sync,
    T::Output: IntoStructuredToolResult,
{
    async fn call(&self) -> Result<CallToolResult, CallToolError> {
        let value = AsyncStructuredTool::call(self)
            .await
            .result()
            .map_err(CallToolError::new)?;
        Ok(
            CallToolResult::text_content(vec![]).with_structured_content(match value {
                serde_json::Value::Object(map) => map,
                value => {
                    let mut map = serde_json::Map::new();
                    map.insert("result".to_string(), value);
                    map
                }
            }),
        )
    }
}

enum CustomToolInner<'a> {
    Text(&'a (dyn CustomTextTool + Send + Sync)),
    Structured(&'a (dyn CustomStructuredTool + Send + Sync)),
    AsyncText(&'a (dyn AsyncCustomTextTool + Send + Sync)),
    AsyncStructured(&'a (dyn AsyncCustomStructuredTool + Send + Sync)),
}

pub struct CustomTool<'a> {
    inner: CustomToolInner<'a>,
}

impl<'a> CustomTool<'a> {
    pub fn text<T, O>(tool: &'a T) -> Self
    where
        T: TextTool<Output = O> + Send + Sync,
        O: IntoTextToolResult,
    {
        Self {
            inner: CustomToolInner::Text(tool),
        }
    }

    pub fn structured<T>(tool: &'a T) -> Self
    where
        T: StructuredTool + Send + Sync,
        T::Output: IntoStructuredToolResult,
    {
        Self {
            inner: CustomToolInner::Structured(tool),
        }
    }

    pub fn async_text<T, O>(tool: &'a T) -> Self
    where
        T: AsyncTextTool<Output = O> + Send + Sync,
        O: IntoTextToolResult,
    {
        Self {
            inner: CustomToolInner::AsyncText(tool),
        }
    }

    pub fn async_structured<T>(tool: &'a T) -> Self
    where
        T: AsyncStructuredTool + Send + Sync,
        T::Output: IntoStructuredToolResult,
    {
        Self {
            inner: CustomToolInner::AsyncStructured(tool),
        }
    }

    pub async fn call(&self) -> Result<CallToolResult, CallToolError> {
        match self.inner {
            CustomToolInner::Text(tool) => tool.call().await,
            CustomToolInner::Structured(tool) => tool.call().await,
            CustomToolInner::AsyncText(tool) => tool.call().await,
            CustomToolInner::AsyncStructured(tool) => tool.call().await,
        }
    }
}
