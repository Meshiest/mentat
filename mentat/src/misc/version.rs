use indexmap::IndexMap;

use super::*;

/// The Version object is utilized to inform the client of the versions of
/// different components of the Rosetta implementation.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Version {
    /// The rosetta_version is the version of the Rosetta interface the
    /// implementation adheres to. This can be useful for clients looking to
    /// reliably parse responses.
    pub rosetta_version: String,
    /// The node_version is the canonical version of the node runtime. This can
    /// help clients manage deployments.
    pub node_version: String,
    /// When a middleware server is used to adhere to the Rosetta interface, it
    /// should return its version here. This can help clients manage
    /// deployments.
    pub middleware_version: Option<String>,
    /// Any other information that may be useful about versioning of dependent
    /// services should be returned here.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
