use super::orderbook::Orderbook;

pub struct TradingPair{
    base: String,
    quote: String,
}

impl TradingPair{
    pub fn new(base: String, quote: String) ->TradingPair {
        TradingPair{
            base,
            quote,
        }
    }
}

pub struct MatchingEngine{
    orderbook: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine{
    pub fn new() -> MatchingEngine{
        MatchingEngine{
            orderbook: HashMap::new(),
        }
    }
}