pub mod wallet;
pub mod provider;
pub mod types;

pub mod client {
    use ethers::{utils, prelude::*};
    use k256::{ecdsa::SigningKey, Secp256k1};

    use super::{
        wallet::wallet::wallet::get_wallet,
        provider::provider::provider::get_provider,
        types::client_types::Client
    };

    pub fn build_client(wallet: &Wallet<SigningKey>, provider: &Provider<Http>) -> Client {
        SignerMiddleware::new(provider.clone(), wallet .clone())
    }
}