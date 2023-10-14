/// QuoteInputParams.0 = sell token
/// QuoteInputParams.1 = (amount, dec)
/// QuoteInputParams.2 = buy token
/// QuoteInputParams.3 = taker address
pub type QuoteInputParams = (Option<String>, Option<(u128, u8)>, Option<String>, Option<String>);

/// OrderBookInputParams.0 = base token
/// OrderBookInputParams.1 = quote token
pub type OrderBookInputParams = (Option<String>, Option<String>);