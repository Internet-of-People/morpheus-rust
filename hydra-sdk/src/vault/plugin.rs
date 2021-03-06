use super::*;

use iop_vault::{Vault, VaultPlugin};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    public_state: Arc<RwLock<PublicState>>,
    parameters: Parameters,
}

#[cfg_attr(target_arch = "wasm32", typetag::serialize(name = "Hydra"))]
#[cfg_attr(not(target_arch = "wasm32"), typetag::serde(name = "Hydra"))]
impl VaultPlugin for Plugin {
    fn name(&self) -> &'static str {
        "Hydra"
    }

    fn to_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn eq(&self, other: &dyn VaultPlugin) -> bool {
        let other: Result<Box<Plugin>, _> = other.to_any().downcast();
        match other {
            Ok(p) => self.parameters == p.parameters,
            Err(_) => false,
        }
    }
}

impl Plugin {
    pub fn new(parameters: Parameters, xpub: String, receive_keys: u32, change_keys: u32) -> Self {
        let public_state = PublicState { xpub, receive_keys, change_keys };
        let public_state = Arc::new(RwLock::new(public_state));
        Self { public_state, parameters }
    }

    fn instantiate(
        vault: &mut Vault, unlock_password: impl AsRef<str>, parameters: &Parameters,
        receive_keys: u32, change_keys: u32,
    ) -> Result<()> {
        let seed = vault.unlock(unlock_password.as_ref())?;
        let account = Self::create_account(parameters, &seed)?;
        let pub_account = account.neuter();
        let plugin =
            Self::new(parameters.to_owned(), pub_account.to_xpub(), receive_keys, change_keys);
        vault.add(Box::new(plugin))
    }

    pub fn create(
        vault: &mut Vault, unlock_password: impl AsRef<str>, parameters: &Parameters,
    ) -> Result<()> {
        Self::instantiate(vault, unlock_password, parameters, 0, 0)
    }

    pub fn init(
        vault: &mut Vault, unlock_password: impl AsRef<str>, parameters: &Parameters,
    ) -> Result<()> {
        Self::instantiate(vault, unlock_password, parameters, 1, 0)
    }

    pub fn get(
        vault: &Vault, parameters: &Parameters,
    ) -> Result<BoundPlugin<Plugin, Public, Private>> {
        let _network = Networks::by_name(&parameters.network)?; // checks if network name is supported
        ensure!(parameters.account >= 0, "Hydra account number cannot be negative");

        let hydra_plugins = vault.plugins_by_type::<Plugin>()?;
        let plugin: &Plugin =
            hydra_plugins
                .iter()
                .by_ref()
                .find(|p| p.parameters == *parameters)
                .with_context(|| "Could not find Hydra plugin with given parameters")?;
        Ok(BoundPlugin::new(vault.to_owned(), plugin.to_owned()))
    }

    pub fn network(&self) -> &'static dyn Network<Suite = Secp256k1> {
        Networks::by_name(&self.parameters.network).unwrap()
    }

    pub fn account(&self) -> i32 {
        self.parameters.account
    }

    pub fn create_account(parameters: &Parameters, seed: &Seed) -> Result<Bip44Account<Secp256k1>> {
        let network = Networks::by_name(&parameters.network)?;
        Bip44.network(seed, network)?.account(parameters.account)
    }

    pub(super) fn to_state(&self) -> Box<dyn State<PublicState>> {
        Box::new(self.public_state.to_owned())
    }
}
