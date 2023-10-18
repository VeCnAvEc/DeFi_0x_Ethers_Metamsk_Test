mod config;
mod dexes;
mod ethers;
mod api;
mod client;
mod tasks;
mod logs;

use std::sync::Arc;

use ::ethers::prelude::{Http, Provider, SignerMiddleware, Wallet, };
use ::ethers::{utils, prelude::*};

use api::ox::params::input_params_type::input_params_type::*;
use api::ox::request::*;

use client::{
    provider::provider::provider::*,
    wallet::wallet::wallet::*,
    types::client_types::*
};
use crate::config::pairs::PairsConfig;
use crate::tasks::look_for_arbitration_opportunity::look_for_arbitration_opportunity::arbitration_opportunity;

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = Arc::new(config::config::EndpointConfig::new());
    let pairs_config = Arc::new(PairsConfig::serialize_pairs_json());

    let provider_result = get_provider();

    if let Err(error) = provider_result {
        println!("code: {}\nmsg: {}", error.0, error.1);
        return Err(error.1);
    }

    let wallet = get_wallet();
    let provider = provider_result.unwrap();

    let client = client::client::build_client(&wallet, &provider);

    arbitration_opportunity(Arc::clone(&pairs_config), provider).await;

    // let order_book_result = order_book::order_book::get_order_book(
    //     order_book_params
    // ).await;

    // if let Err(error) = order_book_result {
    //     return Err(format!("Order book error: {}", error));
    // }

    // println!("quote_result_2 response: {:?}", quote_result_2.unwrap().text().await.unwrap());
    // println!("order_book_result response: {:?}", order_book_result.unwrap().text().await.unwrap());
    // ox::test_0x_api(&client).await;
    // dexes::dexes::send_default_request_in_uniswap(Arc::clone(&config)).await;
    // ethers::ethers_request::check_request(&client, pool_factory).await;
    Ok(())

}