use monte_carlo::option::EuropeanCallOption;

// Helper to avoid repeating all 5 fields in every test.
fn option_with_strike(strike: f32) -> EuropeanCallOption {
    EuropeanCallOption::new(100.0, strike, 1.0, 0.2, 0.05)
}

#[test]
fn payoff_is_zero_when_price_below_strike() {
    let option = option_with_strike(100.0);
    assert_eq!(option.payoff(90.0), 0.0);
}

#[test]
fn payoff_is_zero_when_price_equals_strike() {
    let option = option_with_strike(100.0);
    assert_eq!(option.payoff(100.0), 0.0);
}

#[test]
fn payoff_is_positive_when_price_above_strike() {
    let option = option_with_strike(100.0);
    assert_eq!(option.payoff(120.0), 20.0);
}

#[test]
fn payoff_handles_fractional_values() {
    let option = option_with_strike(50.5);
    let result = option.payoff(75.25);
    assert!((result - 24.75).abs() < 1e-9);
}
