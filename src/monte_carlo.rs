/*
sum = 0

repeat N times:

    generate Z

    calculate S_T

    payoff = max(S_T - K, 0)

    sum += payoff

average = sum / N

price = exp(-rT) * average

*/
use crate::gbm::gbm;
use crate::option::EuropeanCallOption;
use rand::rng;
use rand_distr::{Distribution, StandardNormal};

pub fn price_call(
    option: &EuropeanCallOption,
    num_simulations: i32,
) -> (f32, f32) {
    let mut rng = rng();
    let normal = StandardNormal;
    let mut st_sum  : f32 = 0.0;
    let mut payoff_sum :f32 = 0.0;

    for _ in 0..num_simulations {
        let z: f32 = normal.sample(&mut rng);

        let future_price : f32 = gbm(
            option.spot,
            option.maturity,
            option.volatility,
            option.risk_free_rate,
            z
        );

        let payoff : f32 = option.payoff(future_price);
        payoff_sum += payoff;
        st_sum += future_price;

    }
    let average_payoff :f32 = payoff_sum / num_simulations as f32;
    let average_st :f32 = st_sum / num_simulations as f32;

    return ((-(option.risk_free_rate * option.maturity)).exp() * average_payoff, average_st);

}

