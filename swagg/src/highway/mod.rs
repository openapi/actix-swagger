//! # Highway
//!
//! This module is a database of components and paths
//! Useful when converting OpenAPI structures to printer structures

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct RequestBody {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct ComponentField {
    pub name: String,
    pub required: bool,
    pub description: Option<String>,
}
