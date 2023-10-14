mod config;
mod dexes;
mod ethers;
mod api;
mod client;
mod tasks;

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

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = Arc::new(config::config::EndpointConfig::new());
    let pairs_config = PairsConfig::serialize_pairs_json();

    let provider_result = get_provider();

    if let Err(error) = provider_result {
        println!("code: {}\nmsg: {}", error.0, error.1);
        return Err(error.1);
    }

    let wallet = get_wallet();
    let provider = provider_result.unwrap();

    let client = client::client::build_client(&wallet, &provider);

    let order_book_params: OrderBookInputParams = (
        Some("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string()),
        Some("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string())
    );

    tasks::test_order::tasks_test_order::test_get_two_quotes_and_calculate_benefit_without_commission().await;

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