pub mod config {
    use dotenv_codegen::dotenv;

    #[derive(Debug)]
    pub struct EndpointConfig {
        pub uniswap_graphql_endpoint: String,
    }

    impl EndpointConfig {
        pub fn new() -> Self {
            Self {
                uniswap_graphql_endpoint: dotenv!("UNISWAP_GRAPHQL_ENDPOINT").to_string()
            }
        }
    }
}