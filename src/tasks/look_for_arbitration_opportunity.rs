pub mod look_for_arbitration_opportunity {
    use std::sync::Arc;
    use std::time::Duration;
    use ethers::prelude::{Http, Provider};
    use ethers::providers::Middleware;

    use crate::api::ox::params::input_params_type::input_params_type::QuoteInputParams;
    use crate::api::ox::params::input_params_type::swap_params;
    use crate::api::ox::price::get_price::get_price;
    use crate::config::pairs::PairsConfig;
    use crate::logs::info::{log_order_info, log_token_info};
    use crate::tasks::CommissionAndPaymentParameters;

    pub async fn arbitration_opportunity(pairs_config: Arc<PairsConfig>, provider: Provider<Http>) {
        let mut interval = tokio::time::interval(Duration::from_secs(3));
        let mut request_counter = 0;

        loop {
            request_counter += 1;
            let pairs = pairs_config.get_all_bsc_pair_arc().clone();

            let network_gas_price = provider.get_gas_price().await.unwrap();

            for pair in pairs.iter() {
                let token_buy = pair.get_token0();
                let token_sell = pair.get_token1();

                let params: QuoteInputParams = (
                    Some(token_sell.address.to_string()),
                    Some((15.0, token_sell.decimals)),
                    Some(token_buy.address.to_string()),
                    None
                );

                let response_1 = get_price(&params).await;

                if let Err(error) = &response_1 {
                    println!("Code: {}, Swap price result: {}", error.0, error.1);
                    continue;
                }
                let mut commission_and_payment_params_0 = CommissionAndPaymentParameters::new();
                let token_info_1 = response_1.unwrap();

                if let Err(error) = commission_and_payment_params_0.build(&token_info_1) {
                    println!("code: {}, msg: {}", error.0, error.1);
                    continue;
                }
                let gas_price_in_token_0 = commission_and_payment_params_0.estimated_gas_price_in_coin(network_gas_price);
                commission_and_payment_params_0.set_gas_in_coin(
                    gas_price_in_token_0,
                    "BNB".to_string()
                );

                let swap_params: QuoteInputParams = swap_params(params.clone(), Some(
                        (commission_and_payment_params_0.get_token_price().unwrap() * 15.0, token_buy.decimals)
                ));
                let response_2 = get_price(&swap_params).await;

                if let Err(error) = &response_2 {
                    println!("code: {}, msg: {}", error.0, error.1);
                    continue;
                }

                let token_info_2 = response_2.unwrap();
                let mut commission_and_payment_params_1 = CommissionAndPaymentParameters::new();

                if let Err(error) = commission_and_payment_params_1.build(&token_info_2) {
                    println!("code: {}, msg: {}", error.0, error.1);
                    continue;
                }
                let gas_price_in_token_1 = commission_and_payment_params_1.estimated_gas_price_in_coin(network_gas_price);
                commission_and_payment_params_1.set_gas_in_coin(
                    gas_price_in_token_1,
                    "BNB".to_string()
                );

                // buy amount token 0
                let token_0_purchased_quantity = commission_and_payment_params_0.get_token_price().unwrap() * 15.0;
                let token_0_purchased_quantity_with_price_impact =
                    commission_and_payment_params_0.estimated_with_price_impact().unwrap() * 15.0;
                // buy amount token 1
                let token_1_purchased_quantity = commission_and_payment_params_1.get_token_price().unwrap() * token_0_purchased_quantity;
                let token_1_purchased_quantity_with_price_impact =
                    commission_and_payment_params_1.estimated_with_price_impact().unwrap() * token_0_purchased_quantity;

                if token_1_purchased_quantity >= 15.0 {
                    let log_order_info = log_order_info(1, &token_info_1);
                    println!("-------------pair-------------");
                    println!("INFO: {log_order_info}");
                    let gas_in_coin_info_0_opt = commission_and_payment_params_0.get_gas_in_coin();
                    if let None = gas_in_coin_info_0_opt {
                        println!("the price of gas in tokens is unknown");
                        continue;
                    }
                    let gas_in_coin_info_0 = gas_in_coin_info_0_opt.unwrap();

                    let log_token_info_0 = log_token_info(
                        0, commission_and_payment_params_0.get_sources(),
                        token_buy.symbol.as_str(),
                        commission_and_payment_params_0.get_token_price().unwrap(),
                        commission_and_payment_params_0.estimated_with_price_impact().unwrap(),
                        token_0_purchased_quantity,
                        token_0_purchased_quantity_with_price_impact,
                        commission_and_payment_params_0.get_estimated_gas().unwrap(),
                        gas_in_coin_info_0.1,
                        gas_in_coin_info_0.0.clone(),
                    );

                    let gas_in_coin_info_1_opt = commission_and_payment_params_1.get_gas_in_coin();
                    if let None = gas_in_coin_info_1_opt {
                        println!("the price of gas in tokens is unknown");
                        continue;
                    }
                    let gas_in_coin_info_1 = gas_in_coin_info_1_opt.unwrap();
                    println!("{log_token_info_0}");
                    let log_token_info_1 = log_token_info(
                        1, commission_and_payment_params_1.get_sources(),
                        token_sell.symbol.as_str(),
                        commission_and_payment_params_1.get_token_price().unwrap(),
                        commission_and_payment_params_1.estimated_with_price_impact().unwrap(),
                        token_1_purchased_quantity,
                        token_1_purchased_quantity_with_price_impact,
                        commission_and_payment_params_0.get_estimated_gas().unwrap(),
                        gas_in_coin_info_1.1,
                        gas_in_coin_info_1.0.clone(),
                    );
                    println!(
                        "{log_token_info_1}"
                    );
                    println!("------------------------------\n");
                }
            }

            if request_counter == 3 {
                break;
            }
            interval.tick().await;
        }
    }
}