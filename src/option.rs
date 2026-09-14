// Our financial contract/model parameters


pub struct EuropeanCallOption {
    pub spot: f32,
    pub strike: f32,
    pub maturity: f32,
    pub volatility: f32,
    pub risk_free_rate: f32,
}

impl EuropeanCallOption {
    pub fn new(spot: f32, strike: f32, maturity: f32, volatility: f32, risk_free_rate: f32) -> Self {
        EuropeanCallOption {
            spot,
            strike,
            maturity,
            volatility,
            risk_free_rate,
        }
    }

    pub fn payoff(&self, future_price: f32) -> f32 {
        (future_price - self.strike).max(0.0)
    }
}