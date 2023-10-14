pub mod provider {
    use dotenv_codegen::dotenv;
    use ethers::prelude::{Http, Provider};

    pub fn get_provider() -> Result<Provider<Http>, (i32, String)>{
        let provider_http = format!("{}{}",dotenv!("NODE_API_ENDPOINT_HTTPS"), dotenv!("NODE_API_KEY"));
        let provider = Provider::<Http>::try_from(
            provider_http
        );
        if let Err(error) = &provider {
            println!("ParseError: {}", error.to_string());
            return Err((15432571, error.to_string()));
        }
        Ok(provider.unwrap())
    }
}