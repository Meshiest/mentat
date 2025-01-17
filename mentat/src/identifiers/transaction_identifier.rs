use super::*;

/// The transaction_identifier uniquely identifies a transaction in a particular
/// network and block or in the mempool.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TransactionIdentifier {
    /// Any transactions that are attributable only to a block (ex: a block
    /// event) should use the hash of the block as the identifier.
    pub hash: String,
}
