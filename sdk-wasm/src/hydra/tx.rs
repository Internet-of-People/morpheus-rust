use super::*;

#[wasm_bindgen(js_name = HydraTxBuilder)]
pub struct JsHydraTxBuilder {
    network: &'static dyn Network<Suite = Secp256k1>,
}

#[wasm_bindgen(js_class = HydraTxBuilder)]
impl JsHydraTxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(network_name: &str) -> Result<JsHydraTxBuilder, JsValue> {
        let network = Networks::by_name(network_name).map_err_to_js()?;
        Ok(Self { network })
    }

    // TODO consider recipient SecpKeyId vs String
    pub fn transfer(
        &self, recipient_id: &JsSecpKeyId, sender_pubkey: &JsSecpPublicKey, amount_flake: u64,
        nonce: u64, vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { amount: amount_flake, vendor_field, manual_fee },
        };

        let transfer = hyd_core::Transaction::transfer(common_fields, recipient_id.inner());
        JsValue::from_serde(&transfer.to_data()).map_err_to_js()
    }

    pub fn vote(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        self.create_vote_tx(
            delegate,
            sender_pubkey,
            nonce,
            vendor_field,
            manual_fee,
            hyd_core::Transaction::vote,
        )
    }

    pub fn unvote(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        self.create_vote_tx(
            delegate,
            sender_pubkey,
            nonce,
            vendor_field,
            manual_fee,
            hyd_core::Transaction::unvote,
        )
    }

    #[wasm_bindgen(js_name = registerDelegate)]
    pub fn register_delegate(
        &self, sender_pubkey: &JsSecpPublicKey, delegate_name: &str, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { vendor_field, manual_fee, ..Default::default() },
        };

        let tx = hyd_core::Transaction::register_delegate(common_fields, delegate_name);
        JsValue::from_serde(&tx.to_data()).map_err_to_js()
    }

    fn create_vote_tx(
        &self, delegate: &JsSecpPublicKey, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
        build_tx: fn(
            CommonTransactionFields<'static>,
            &SecpPublicKey,
        ) -> hyd_core::Transaction<'static>,
    ) -> Result<JsValue, JsValue> {
        let common_fields = CommonTransactionFields {
            network: self.network,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { vendor_field, manual_fee, ..Default::default() },
        };

        let vote = build_tx(common_fields, delegate.inner());
        JsValue::from_serde(&vote.to_data()).map_err_to_js()
    }
}
