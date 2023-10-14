pub mod wallet {
    use dotenv_codegen::dotenv;
    use ethers::{utils, prelude::*};
    use ethers::types::Chain;

    pub fn get_wallet() -> LocalWallet {
        let wallet: LocalWallet = dotenv!("WALLET_PRIVATE_KEY")
            .parse::<LocalWallet>().unwrap()
            .with_chain_id(Chain::BinanceSmartChain);
        return wallet;
    }
}