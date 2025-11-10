use mcp_utils::tool_prelude::*;

/// Sums a list of numbers together.
#[mcp_tool(
    name = "sum",
    description = concat!(
        "Calculates the sum of a list of numbers. Returns the total sum or an error if ",
        "the result would be infinite or invalid."
    ),
    title = "Sum numbers together",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true,
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SumTool {
    pub values: Vec<f64>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SumResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl StructuredTool for SumTool {
    type Output = SumResult;

    fn call(&self) -> Self::Output {
        let mut sum = 0.0_f64;

        for number in self.values.iter() {
            let new_sum = sum + number;
            if new_sum.is_finite() {
                sum = new_sum;
            } else {
                return SumResult {
                    error: Some("Infinite value detected".to_string()),
                    sum: None,
                };
            }
        }

        SumResult {
            sum: Some(sum),
            error: None,
        }
    }
}
