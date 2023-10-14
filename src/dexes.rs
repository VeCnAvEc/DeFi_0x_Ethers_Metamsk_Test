pub mod dexes {
    use std::sync::Arc;
    use crate::config::config::EndpointConfig;
    use serde_json::{json, Value};

    pub async fn send_default_request_in_uniswap(config: Arc<EndpointConfig>) {
        let client = reqwest::Client::new();
        let graphql_query = r#"
            {
              pool(id: "0x5777d92f208679db4b9778590fa3cab3ac9e2168") {
                liquidity
                token0 {
                  decimals
                  id
                  symbol
                  name
                }
                token1 {
                  decimals
                  id
                  name
                  symbol
                }
                token0Price
                token1Price
              }
            }
        "#;
        let json_request = json!({
            "query": graphql_query
        });

        let response = client
            .post(&config.uniswap_graphql_endpoint)
            .json(&json_request)
            .send()
            .await.unwrap()
            .json::<Value>()
            .await.unwrap();

        println!("response: {:?}", response)
    }
}