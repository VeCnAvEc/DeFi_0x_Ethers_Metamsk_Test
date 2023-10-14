pub mod tasks_test_order {
    use std::time::Duration;

    use serde_json::Value;

    use crate::api::ox::params::input_params_type::input_params_type::QuoteInputParams;
    use crate::api::ox::request::quote;

    pub async fn test_get_two_quotes_and_calculate_benefit_without_commission() {
        let mut interval = tokio::time::interval(Duration::from_secs(3));
        loop {
            // test data
            let quote_params_2: QuoteInputParams = (
                // SELL TOKEN
                Some("0x6ad9E9c098a45B2B41b519119C31c3DcB02ACcB2".to_string()), // PZP
                Some((1, 18)),
                // BUY TOKEN
                Some("0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56".to_string()), // BUSD
                None
                // Some("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string())
            );

            let quote_params_1: QuoteInputParams = (
                // SELL TOKEN
                Some("0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56".to_string()), // BUSD
                Some((1, 18)),
                // BUY TOKEN
                Some("0x6ad9E9c098a45B2B41b519119C31c3DcB02ACcB2".to_string()), // PZP
                None
                // Some("0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string())
            );
            //

            let price_result = quote::quote::swap_quote( &quote_params_1, "swap/v1/price?").await;

            println!("{}", price_result.unwrap().text().await.unwrap());

            let quote_result_2 = quote::quote::swap_quote(
                &quote_params_2,
                "swap/v1/quote?"
            ).await;
            let quote_result_1 = quote::quote::swap_quote(
                &quote_params_1,
                "swap/v1/quote?"
            ).await;
            if let Err(error) = quote_result_1 {
                println!("Error msg: {}", error);
                continue;
            }
            if let Err(error) = quote_result_2 {
                println!("Error msg: {}", error);
                continue;
            }

            let best_order_1 = quote_result_1.unwrap().text().await.unwrap();
            let best_order_2 = quote_result_2.unwrap().text().await.unwrap();

            let order_to_json_1: Value = serde_json::from_str(best_order_1.as_str()).unwrap();
            let order_to_json_2: Value = serde_json::from_str(best_order_2.as_str()).unwrap();

            let price_1 = order_to_json_1.get("price").unwrap();
            let price_2 = order_to_json_2.get("price").unwrap();
            println!("price_1: {}", price_1);
            println!("price_2: {}", price_2);

            let buy_bnb = price_1.as_str().unwrap().parse::<f64>().unwrap() * 200.0;
            let buy_usdt = buy_bnb * price_2.as_str().unwrap().parse::<f64>().unwrap();
            println!("price: {}, amount usdt: 200, buy_bnb: {}", price_1, buy_bnb);
            println!("price: {}, amount wbnb: {buy_bnb}, buy_usdt: {}", price_2, buy_usdt);

            interval.tick().await;
        }
    }
}