pub mod order_book {
    use dotenv_codegen::dotenv;
    use reqwest::{Response, Client};

    use crate::api::ox::{params::{order_book_params::OrderBookParams, params_inerface::OxRequestParams, input_params_type::input_params_type::OrderBookInputParams}};
    use crate::api::ox::request::{ BSC_0X_URL, MAINNET_0X_URL};

    pub async fn get_order_book(input_params: OrderBookInputParams) -> Result<Response, String> {
        let mut url = BSC_0X_URL.to_string();
        url.push_str("orderbook/v1?");

        let mut params = OrderBookParams::new();    
        params.set_params(
            &input_params
        );
        
        let query = params.build_quary_for_request();

        if let Err(error) = query {
            return Err(error);
        }

        url.push_str(&*query.unwrap());

        let client = Client::new();
        
        let ox_token = dotenv!("OX_API_TOKEN");
        let response_result = client.get(url).header("0x-api-key", ox_token).send().await;

        if let Err(error) = response_result {
            return Err(error.to_string());
        }

        Ok(response_result.unwrap())
    }   
}