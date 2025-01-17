use super::*;

#[derive(Clone, Default)]
pub struct DummyIndexerApi;

#[axum::async_trait]
impl CallerIndexerApi for DummyIndexerApi {}

#[axum::async_trait]
impl IndexerApi for DummyIndexerApi {}
