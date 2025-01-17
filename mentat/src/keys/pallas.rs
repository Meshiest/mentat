use std::marker::PhantomData;

use mina_hasher::{Hashable, ROInput};
use mina_signer::{create_legacy, CompressedPubKey, Keypair, NetworkId, Signature, Signer};

use super::{Keys, KeysError};

pub struct PallasKeys<H: Hashable> {
    keypair: Keypair,
    _phantom_data: PhantomData<H>,
}

impl<H: Hashable> Keys for PallasKeys<H> {
    type M = Transaction;
    type S = Signature;

    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        let hex_string =
            std::str::from_utf8(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        let keypair =
            Keypair::from_hex(hex_string).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        Ok(Self {
            keypair,
            _phantom_data: PhantomData::default(),
        })
    }

    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
        let mut signer = create_legacy::<Self::M>(NetworkId::MAINNET);
        Ok(signer.sign(&self.keypair, message))
    }

    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
        let mut signer = create_legacy::<Self::M>(NetworkId::MAINNET);
        Ok(signer.verify(signature, &self.keypair.public, message))
    }
}

// Mina signature scheme typically initializes its keys for specific
// structures. This transaction structure is an identical copy of the Mina one,
// and was taken from: https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs

const MEMO_BYTES: usize = 34;
const TAG_BITS: usize = 3;

#[derive(Clone, Copy)]
pub struct Transaction {
    // Common
    pub fee: u64,
    pub fee_token: u64,
    pub fee_payer_pk: CompressedPubKey,
    pub nonce: u32,
    pub valid_until: u32,
    pub memo: [u8; MEMO_BYTES],
    // Body
    pub tag: [bool; TAG_BITS],
    pub source_pk: CompressedPubKey,
    pub receiver_pk: CompressedPubKey,
    pub token_id: u64,
    pub amount: u64,
    pub token_locked: bool,
}

impl Hashable for Transaction {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        roi.append_field(self.fee_payer_pk.x);
        roi.append_field(self.source_pk.x);
        roi.append_field(self.receiver_pk.x);

        roi.append_u64(self.fee);
        roi.append_u64(self.fee_token);
        roi.append_bool(self.fee_payer_pk.is_odd);
        roi.append_u32(self.nonce);
        roi.append_u32(self.valid_until);
        roi.append_bytes(&self.memo);

        for tag_bit in self.tag {
            roi.append_bool(tag_bit);
        }

        roi.append_bool(self.source_pk.is_odd);
        roi.append_bool(self.receiver_pk.is_odd);
        roi.append_u64(self.token_id);
        roi.append_u64(self.amount);
        roi.append_bool(self.token_locked);

        roi
    }

    fn domain_string(_: Option<&Self>, network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}
