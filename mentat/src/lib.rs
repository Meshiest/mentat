pub mod errors;
pub mod identifiers;
pub mod misc;
pub mod models;
pub mod requests;
pub mod responses;

#[cfg(feature = "sdk")]
pub mod keys;

#[cfg(feature = "server")]
#[path = ""]
mod server_rexport {
    pub mod api;
    pub mod cache;
    pub mod conf;
    pub mod server;

    pub use axum::{self, async_trait, Json};
    pub use indexmap::IndexMap;
    pub use mentat_macros::{main, mentat};
    pub use serde;
    pub use serde_json;
    pub use tokio;
    pub use tracing;

    pub mod macro_exports {
        pub use axum::{
            extract::{self, ConnectInfo, Extension, Json},
            routing,
            Router,
        };
        pub use tracing::Instrument;

        pub use super::{api::*, cache::Cache, conf::Configuration, server::RpcCaller, *};
        pub use crate::requests::*;
    }
}

#[cfg(feature = "server")]
pub use server_rexport::*;

#[cfg(feature = "client")]
pub mod client;
