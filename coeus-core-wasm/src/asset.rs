use super::*;

#[wasm_bindgen(js_name = CoeusAsset)]
pub struct JsCoeusAsset {
    inner: CoeusAsset,
}

#[wasm_bindgen(js_class = CoeusAsset)]
impl JsCoeusAsset {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue) -> Result<JsCoeusAsset, JsValue> {
        let data_serde: serde_json::Value = data.into_serde().map_err_to_js()?;
        let inner: CoeusAsset = serde_json::from_value(data_serde).map_err_to_js()?;
        Ok(inner.into())
    }

    pub fn deserialize(bytes: &[u8]) -> Result<JsCoeusAsset, JsValue> {
        let asset_str = IopTransactionType::protobuf_to_string(bytes).map_err_to_js()?;
        let inner = serde_json::from_str(&asset_str).map_err_to_js()?;
        Ok(JsCoeusAsset { inner })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, JsValue> {
        self.inner.to_bytes().map_err_to_js()
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.inner).map_err_to_js()
    }
}

impl From<CoeusAsset> for JsCoeusAsset {
    fn from(inner: CoeusAsset) -> Self {
        Self { inner }
    }
}

impl Wraps<CoeusAsset> for JsCoeusAsset {
    fn inner(&self) -> &CoeusAsset {
        &self.inner
    }
}