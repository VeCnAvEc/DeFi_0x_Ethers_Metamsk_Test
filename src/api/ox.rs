pub mod request;
pub mod params;

use dotenv_codegen::dotenv;
use ethers::{prelude::*};
use ethers::prelude::{Http, Provider};
use ethers::providers::Middleware;
use ethers::types::Transaction;
use ethers::utils::ParseUnits;
use serde::{Deserialize, Serialize}; 
use crate::client::types::client_types::Client;
use serde_json::Value;

const OX_URL_API: &str = "https://api.0x.org/";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Params {  
    sellToken: &'static str,
    buyToken: &'static str,
    sellAmount: String,
}

pub async fn test_0x_api(client: &Client) {
    let http_client = reqwest::Client::new();

    let ox_token =  dotenv!("OX_API_TOKEN");
    let mut url = OX_URL_API.to_string();
    let params = get_params(1);

    let query = serde_qs::to_string(&params).unwrap();
    url.push_str("swap/v1/quote?");
    url.push_str(query.as_str());
 
    println!("url: {}", url);

    let response = http_client.get(url).header("0x-api-key", ox_token).send().await.unwrap();
    let response_text = response.text().await.unwrap();
    let response_to_json: Value = serde_json::from_str(response_text.as_str()).unwrap();

    if let Err(error) = check_on_err(&response_to_json) {
        println!("error: {:?}", error);
        return;
    }

    let mut tx = TransactionRequest::default();
    let gas: u64 = response_to_json.get("gas").unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("0").parse().unwrap_or(0);
    let gas_price: u64 = response_to_json.get("gasPrice").unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("0").parse().unwrap_or(0);
    let to: Address = response_to_json.get("to").unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("").parse().unwrap_or(Address::default());
    let value: u128 = response_to_json.get("value").unwrap_or(&Value::String("".to_string())).as_str().unwrap_or("0").parse().unwrap_or(0);
    let chain_id: u16 = response_to_json.get("chainId").unwrap_or(&Value::String("".to_string())).to_string().parse().unwrap_or(0);

    println!("chain_id: {}", chain_id);

    tx.gas = Some(U256::from(gas));
    tx.gas_price = Some(U256::from(gas_price));
    tx.to = Some(ethers::types::NameOrAddress::Address(to));
    tx.value = Some(U256::from(value));
    tx.chain_id = Some(U64::from(chain_id));

    let tx_result = client.send_transaction(tx,
        None
    ).await;

    if let Err(error) = tx_result {
        println!("Error: {:?}", error.to_string()); 
    }
}

pub fn get_params(amount: u128) -> Params {
    let amount = ::ethers::utils::parse_units(amount, 18).unwrap();
    println!("amount: {:?}", amount);
    let params = Params {
        sellToken: "0x6B175474E89094C44Da98b954EedeAC495271d0F",  //DAI
        buyToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", 
        sellAmount: amount.to_string(),
    };

    params
}

fn check_on_err(response: &Value) -> Result<(), (i32, String)> {
    let code = response.get("code");
    let reason = response.get("reason");
    let values = match response.get("values") {
        Some(value) => {
            let mut message = None;

            if let Some(msg) = value.get("message") {
                message = Some(msg);
            }
            message
        },
        None => {
            None
        }
    };

    if code.is_some() && reason.is_some() && values.is_some() {
        let b = values.unwrap().to_string();
        let a = code.unwrap().to_string().parse::<i32>().unwrap();
        return Err((a, b))
    }

    Ok(())
}