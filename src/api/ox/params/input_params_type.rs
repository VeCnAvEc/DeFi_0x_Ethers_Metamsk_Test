use crate::api::ox::params::input_params_type::input_params_type::QuoteInputParams;
use crate::config::pairs::PairsConfig;

pub mod input_params_type;

pub fn swap_params(params: QuoteInputParams, ad: Option<(f64, u8)>) -> QuoteInputParams {
    let swap_params: QuoteInputParams = (
        params.2,
        ad,
        params.0,
        params.3
    );
    swap_params
}