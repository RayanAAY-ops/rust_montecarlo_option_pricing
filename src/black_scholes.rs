use crate::option::EuropeanCallOption;

/// Standard normal cumulative distribution function N(x).
///
/// N(x) = 1/2 * (1 + erf(x / sqrt(2)))
fn normal_cdf(x: f32) -> f32 {
    0.5 * (1.0 + libm::erff(x / std::f32::consts::SQRT_2))
}

/// C = S * N(d1) - K * e^(-rT) * N(d2)
pub fn price_call(option: &EuropeanCallOption) -> f32 {
    let d1 = ((option.spot / option.strike).ln()
        + (option.risk_free_rate + 0.5 * option.volatility.powi(2)) * option.maturity)
        / (option.volatility * option.maturity.sqrt());

    let d2 = d1 - option.volatility * option.maturity.sqrt();

    option.spot * normal_cdf(d1)
        - option.strike * (-option.risk_free_rate * option.maturity).exp() * normal_cdf(d2)
}
