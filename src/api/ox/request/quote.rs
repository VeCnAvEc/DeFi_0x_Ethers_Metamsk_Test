pub mod quote {
    use dotenv_codegen::dotenv;
    use reqwest::{Response, Client};

    use crate::api::ox::{params::{quote_params::QuoteParams, params_inerface::OxRequestParams, input_params_type::input_params_type::QuoteInputParams}, request::MAINNET_0X_URL};
    use crate::api::ox::request::BSC_0X_URL;

    pub async fn swap_quote(
        input_params: &QuoteInputParams,
        uri: &str
    ) -> Result<Response, String> {
        let mut params = QuoteParams::new();
        params.set_params(&input_params);
        let query = params.build_quary_for_request();

        println!("query: {:?}", query);

        if let Err(error) = query {
            println!("query error: {:?}", error);
            return Err(error);
        }

        let mut url = BSC_0X_URL.to_string();
        url.push_str(uri);
        url.push_str(query.unwrap().as_str());

        let ox_token = dotenv!("OX_API_TOKEN");
        let client = Client::new();
        let response = client.get(url).header("0x-api-key", ox_token).send().await;
        
        if let Err(error_response)= response {
            return Err(error_response.to_string());
        }

        Ok(response.unwrap())
    }
}