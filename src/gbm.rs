pub fn gbm(spot: f32,
    maturity: f32,
    volatility: f32,
    risk_free_rate: f32,
    z: f32,
) -> f32 {
    let drift = risk_free_rate - 0.5 * volatility.powi(2);
    let diffusion = volatility * maturity * z;
    spot * (drift + diffusion).exp()
}