//! Debug State Object (DSO) - structured error reporting

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Debug State Object for structured error reporting
///
/// DSO provides AI-parseable error information in JSON format,
/// allowing for self-correction without parsing natural language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugStateObject {
    /// Status of the error
    pub status: String,

    /// Machine-readable error code
    pub error_code: String,

    /// Location reference (content-addressed node ID)
    pub location: Option<String>,

    /// Context information (variable values, memory snapshot, etc.)
    pub context: Value,

    /// Expected value/expression
    pub expected: Option<String>,

    /// Actual value
    pub actual: Option<String>,

    /// Suggested fix
    pub suggestion: Option<String>,

    /// Additional metadata
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl DebugStateObject {
    /// Create a new empty DSO
    pub fn new(status: String, error_code: String) -> Self {
        Self {
            status,
            error_code,
            location: None,
            context: Value::Object(Map::new()),
            expected: None,
            actual: None,
            suggestion: None,
            extra: Map::new(),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Parse from JSON string
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

/// Builder for creating DSOs
#[derive(Debug, Clone, Default)]
pub struct DsoBuilder {
    status: Option<String>,
    error_code: Option<String>,
    location: Option<String>,
    context: Value,
    expected: Option<String>,
    actual: Option<String>,
    suggestion: Option<String>,
    extra: Map<String, Value>,
}

impl DsoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn error_code(mut self, code: impl Into<String>) -> Self {
        self.error_code = Some(code.into());
        self
    }

    pub fn location(mut self, loc: impl Into<String>) -> Self {
        self.location = Some(loc.into());
        self
    }

    pub fn context(mut self, ctx: Value) -> Self {
        self.context = ctx;
        self
    }

    pub fn expected(mut self, exp: impl Into<String>) -> Self {
        self.expected = Some(exp.into());
        self
    }

    pub fn actual(mut self, act: impl Into<String>) -> Self {
        self.actual = Some(act.into());
        self
    }

    pub fn suggestion(mut self, sugg: impl Into<String>) -> Self {
        self.suggestion = Some(sugg.into());
        self
    }

    pub fn extra(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }

    pub fn build(self) -> DebugStateObject {
        DebugStateObject {
            status: self.status.unwrap_or_else(|| "Unknown".to_string()),
            error_code: self.error_code.unwrap_or_else(|| "UNKNOWN".to_string()),
            location: self.location,
            context: self.context,
            expected: self.expected,
            actual: self.actual,
            suggestion: self.suggestion,
            extra: self.extra,
        }
    }
}

/// Standard error codes
pub mod error_codes {
    pub const CONSTRAINT_VIOLATION: &str = "CONSTRAINT_VIOLATION";
    pub const PRECONDITION_VIOLATION: &str = "PRECONDITION_VIOLATION";
    pub const POSTCONDITION_VIOLATION: &str = "POSTCONDITION_VIOLATION";
    pub const TYPE_ERROR: &str = "TYPE_ERROR";
    pub const UNDEFINED_REFERENCE: &str = "UNDEFINED_REFERENCE";
    pub const RUNTIME_ERROR: &str = "RUNTIME_ERROR";
    pub const DIVISION_BY_ZERO: &str = "DIVISION_BY_ZERO";
    pub const INDEX_OUT_OF_BOUNDS: &str = "INDEX_OUT_OF_BOUNDS";
    pub const INVALID_MEMORY_ACCESS: &str = "INVALID_MEMORY_ACCESS";
}
