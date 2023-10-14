pub mod quote {
    use dotenv_codegen::dotenv;
    use reqwest::{Response, Client};

    use crate::api::ox::{params::{quote_params::QuoteParams, params_inerface::OxRequestParams, input_params_type::input_params_type::QuoteInputParams}, request::MAINNET_0X_URL};
    use crate::api::ox::request::BSC_0X_URL;

    pub async fn swap_quote(
        params_info: QuoteInputParams
    ) -> Result<Response, String> {
        let mut params = QuoteParams::new();
        params.set_params(
            &params_info
        );
        let quary = params.build_quary_for_request();

        println!("quary: {:?}", quary);

        if let Err(error) = quary {
            println!("quary errorL: {:?}", error);
            return Err(error);
        }

        let mut url = BSC_0X_URL.to_string();
        url.push_str("swap/v1/quote?");
        url.push_str(quary.unwrap().as_str());

        let ox_token = dotenv!("OX_API_TOKEN");
        let client = Client::new();
        let response = client.get(url).header("0x-api-key", ox_token).send().await;
        
        if let Err(error_response)= response {
            return Err(error_response.to_string());
        }

        Ok(response.unwrap())
    }
}