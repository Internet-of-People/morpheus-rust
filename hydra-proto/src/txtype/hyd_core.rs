use super::*;

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, FromPrimitive, Hash, PartialEq, Serialize_repr,
)]
#[repr(u16)]
pub enum CoreTransactionType {
    Transfer = 0,
    SecondSignatureRegistration = 1,
    DelegateRegistration = 2,
    Vote = 3,
    MultiSignatureRegistration = 4,
    Ipfs = 5,
    TimelockTransfer = 6,
    MultiPayment = 7,
    DelegateResignation = 8,
}

impl Default for CoreTransactionType {
    fn default() -> Self {
        Self::Transfer
    }
}

impl CoreTransactionType {
    pub const TYPE_GROUP: u32 = 1;

    pub fn fee(self) -> u64 {
        match self {
            Self::Transfer => 10_000_000,
            Self::SecondSignatureRegistration => 500_000_000,
            Self::DelegateRegistration => 2_500_000_000,
            Self::Vote => 100_000_000,
            Self::MultiSignatureRegistration => 500_000_000,
            Self::Ipfs => 0,
            Self::TimelockTransfer => 0,
            Self::MultiPayment => 0,
            Self::DelegateResignation => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoreAsset {
    None,
    Signature {
        #[serde(rename = "publicKey")]
        public_key: String,
    },
    Delegate {
        username: String,
    },
    Votes(Vec<String>),
    #[serde(rename = "multiSignature")]
    MultiSignatureRegistration {
        #[serde(rename = "publicKeys")]
        public_keys: Vec<String>,
        min: u8,
    },
    Ipfs(String),
    Payments(Vec<PaymentsItem>),
    Lock {
        #[serde(rename = "secretHash")]
        secret_hash: String,
        expiration: LockExpiration,
    },
    Claim {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
        #[serde(rename = "unlockSecret")]
        unlock_secret: String,
    },
    #[serde(rename = "refund")]
    Refund {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
    },
    #[serde(rename = "businessRegistration")]
    BusinessRegistration {
        name: String,
        website: String,
    },
    #[serde(rename = "businessUpdate")]
    BusinessUpdate {
        name: String,
        website: String,
    },
    #[serde(rename = "bridgechainRegistration")]
    BridgeChainRegistration {
        name: String,
        #[serde(rename = "seedNodes")]
        seed_nodes: Vec<String>,
        #[serde(rename = "genesisHash")]
        genesis_hash: String,
        #[serde(rename = "bridgechainRepository")]
        bridgechain_repository: String,
        ports: HashMap<String, u32>,
    },
    #[serde(rename = "bridgechainUpdate")]
    BridgechainUpdate {
        #[serde(rename = "bridgechainId")]
        bridgechain_id: String,
        #[serde(rename = "seedNodes")]
        seed_nodes: Vec<String>,
        ports: HashMap<String, u32>,
    },
    #[serde(rename = "bridgechainResignation")]
    BridgechainResignation {
        #[serde(rename = "bridgechainId")]
        bridgechain_id: String,
    },
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockExpiration {
    #[serde(rename = "type")]
    pub expiration_type: u64,
    pub value: u64,
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsItem {
    pub amount: String,
    pub recipient_id: String,
}

impl CoreAsset {
    pub fn is_none(&self) -> bool {
        matches!(*self, CoreAsset::None)
    }
}

impl Default for CoreAsset {
    fn default() -> Self {
        CoreAsset::None
    }
}

#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    common_fields: CommonTransactionFields<'a>,
    tx_type: CoreTransactionType,
    asset: CoreAsset,
    recipient_id: Option<SecpKeyId>,
}

impl<'a> Transaction<'a> {
    pub fn transfer(common_fields: CommonTransactionFields<'a>, recipient_id: &SecpKeyId) -> Self {
        Self {
            common_fields,
            tx_type: CoreTransactionType::Transfer,
            recipient_id: Some(recipient_id.to_owned()),
            asset: CoreAsset::None,
        }
    }

    pub fn register_delegate(
        common_fields: CommonTransactionFields<'a>, delegate_name: &str,
    ) -> Self {
        Self {
            common_fields,
            tx_type: CoreTransactionType::DelegateRegistration,
            recipient_id: None,
            asset: CoreAsset::Delegate { username: delegate_name.to_owned() },
        }
    }

    pub fn vote<'b>(
        common_fields: CommonTransactionFields<'a>, delegate: &'b SecpPublicKey,
    ) -> Self {
        Self::create_vote(common_fields, format!("+{}", delegate))
    }

    pub fn unvote<'b>(
        common_fields: CommonTransactionFields<'a>, delegate: &'b SecpPublicKey,
    ) -> Self {
        Self::create_vote(common_fields, format!("-{}", delegate))
    }

    fn create_vote(common_fields: CommonTransactionFields<'a>, vote: String) -> Self {
        Self {
            common_fields,
            tx_type: CoreTransactionType::Vote,
            recipient_id: None,
            asset: CoreAsset::Votes(vec![vote]),
        }
    }
}

impl<'a> Aip29Transaction for Transaction<'a> {
    fn fee(&self) -> u64 {
        self.tx_type.fee()
    }

    fn to_data(&self) -> TransactionData {
        let prefix = self.common_fields.network.p2pkh_addr();

        let mut tx_data: TransactionData = self.common_fields.to_data();
        let core_typedasset = (self.tx_type, self.asset.to_owned());
        tx_data.typed_asset = core_typedasset.into();
        tx_data.recipient_id = self.recipient_id.as_ref().map(|addr| addr.to_p2pkh_addr(prefix));
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}
