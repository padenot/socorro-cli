use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    #[serde(default)]
    pub frame: u32,
    pub function: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub module: Option<String>,
    pub offset: Option<String>,
}
