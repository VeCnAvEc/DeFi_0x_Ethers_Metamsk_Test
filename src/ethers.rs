pub mod ethers_request {
    use std::sync::Arc;
    use ethers::abi::Address;
    use ethers::{prelude::*};
    use crate::client::types::client_types::Client;

    use ::ethers::prelude::abigen;

    abigen!(
        UniswapPoolV3,
        "./src/abi/UniswapV3Pool_ABI.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );

    abigen!(
        UnswapFactoryV3,
        "./src/abi/UniswapV3Factory_ABI.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );

    pub async fn check_request(client: &Client, pool_factory: Address) {
        let pool_usdc_eth = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".parse::<Address>().unwrap();
        get_pool(client, pool_factory).await;
    }

    async fn get_pool(client: &Client, pool_factory: Address) {
        let contract = UnswapFactoryV3::new(pool_factory, Arc::new(client.clone()));
        println!("contract: {:?}", contract);
        let tx = contract.get_pool(
            "0x6B175474E89094C44Da98b954EedeAC495271d0F".parse::<Address>().unwrap(),
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>().unwrap(),
            100
        ).send().await.unwrap().await.unwrap();

        println!("Transaction Receipt: {:?}", serde_json::to_string(&tx));
    }

    // pub async fn increment_number(client: &Client, contract_addr: &H160) -> Result<(), Box<dyn std::error::Error>> {
    //     println!("Incrementing number...");
    //
    //     let contract = Incrementer::new(*contract_addr, Arc::new(client.clone()));
    //
    //     let tx = contract.increment(U256::from(5)).send().await.unwrap().await.unwrap();
    //     println!("Transaction Receipt: {}", serde_json::to_string(&tx)?);
    //
    //     Ok(())
    // }
}