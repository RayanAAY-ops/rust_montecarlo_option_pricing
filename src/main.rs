
use monte_carlo::option::EuropeanCallOption;
use monte_carlo::monte_carlo::price_call;
use monte_carlo::gbm::gbm;
use monte_carlo::black_scholes;
fn main() {

    let spot = 100.0;     // S0 spot price, the current price of the underlying asset
    let strike = 100.0;   // K strike price, the price of the underlying asset at which the option can be exercised
    let maturity = 1.0;  // T time to maturity, the time until the option expires
    let volatility = 0.20; // sigma, the volatility of the stock
    let risk_free_rate = 0.05; //theoretical risk-free rate, the rate of return on a risk-free investment

    let option = EuropeanCallOption {
        spot,
        strike,
        maturity,
        volatility,
        risk_free_rate,
    };

    let (option_price, terminal_price) = price_call(&option, 1_000_000);
    let bs_price = black_scholes::price_call(&option);

    println!("Monte Carlo price:   {:.4}", option_price);
    println!("Black-Scholes price: {:.4}", bs_price);
    println!("The terminal price of the underlying asset is: {:.4}", terminal_price);
}
