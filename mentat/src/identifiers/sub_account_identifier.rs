use indexmap::IndexMap;

use super::*;

/// An account may have state specific to a contract address (ERC-20 token)
/// and/or a stake (delegated balance). The sub_account_identifier should
/// specify which state (if applicable) an account instantiation refers to.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SubAccountIdentifier {
    /// The SubAccount address may be a cryptographic value or some other
    /// identifier (ex: bonded) that uniquely specifies a SubAccount.
    pub address: String,
    /// If the SubAccount address is not sufficient to uniquely specify a
    /// SubAccount, any other identifying information can be stored here. It is
    /// important to note that two SubAccounts with identical addresses but
    /// differing metadata will not be considered equal by clients.
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
