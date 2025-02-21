use serde::{Deserialize, Serialize};

/// These Static variables represents action,triggers,rules and namespaces endpoints
pub static ACTION_ENDPOINT: &str = "actions";
pub static TRIGGERS_ENDPOINT: &str = "triggers";
pub static RULES_ENDPOINT: &str = "rules";
pub static NAMESPACE_ENDPOINT: &str = "namespaces";

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Limits {
    /// Timeout is the range set for per action in milliseconds
    #[serde(default)]
    pub timeout: i64,
    /// Memory is the range set per action in MB
    #[serde(default)]
    pub memory: i64,
    /// The size for the log file
    #[serde(default)]
    pub logsize: i64,
    /// Number of activations that can be processed at once
    #[serde(default)]
    pub concurrency: i64,
}

/// Enum of HTTP Methods
pub enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
}
