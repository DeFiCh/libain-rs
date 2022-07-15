use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref COEFF_DEX_FEE: RwLock<f64> = RwLock::new(1.8);
    static ref COEFF_DISCOUNT: RwLock<f64> = RwLock::new(500.0);
    static ref COEFF_PREMIUM: RwLock<f64> = RwLock::new(3.4);
}

#[cxx::bridge]
mod calc {
    extern "Rust" {
        fn set_fee_coefficient(coefficient: f64);

        fn set_interest_rate_coefficients(discount: f64, premium: f64);

        fn calc_dex_fee(algo_dusd: f64, dusd_supply: f64) -> f64;

        fn calc_loan_interest_rate(
            reserve_dfi: f64,
            reserve_dusd: f64,
            dfi_oracle_price: f64,
        ) -> f64;
    }
}

/// **DFIP-2206-D**
///
/// Sets the coefficient for calculation of dex stabilization fee. Default is `1.8`
fn set_fee_coefficient(coefficient: f64) {
    *COEFF_DEX_FEE.write().unwrap() = coefficient;
}

/// **DFIP-2206-E**
///
/// Sets the coefficients for discount and premium of DUSD for calculation of interest rates.
/// Default is `500` for discount and `3.4` for premium.
fn set_interest_rate_coefficients(discount: f64, premium: f64) {
    *COEFF_DISCOUNT.write().unwrap() = discount;
    *COEFF_PREMIUM.write().unwrap() = premium;
}

/// **DFIP-2206-D**
///
/// Calculates the dynamic dex stabilization fee for DUSD on DUSD-DFI pair. The size of the fee
/// is determined by the ratio of algorithmic DUSD to the total amount of outstanding DUSD.
fn calc_dex_fee(algo_dusd: f64, dusd_supply: f64) -> f64 {
    if dusd_supply <= 0.0 {
        log::warn!("DUSD supply must be positive. Received {}", dusd_supply);
        return 0.0;
    }
    let ratio = 1.0 - (algo_dusd / dusd_supply);
    let coeff = *COEFF_DEX_FEE.read().unwrap();
    if ratio > 0.5 {
        coeff.powf(ratio - 0.5) - 1.0
    } else {
        0.0
    }
}

/// **DFIP-2206-E**
///
/// Calculates the dynamic interest rates on DUSD loans, based on the current discount/premium
/// of DUSD evaluated with the DFI price oracle.
fn calc_loan_interest_rate(reserve_dfi: f64, reserve_dusd: f64, dfi_oracle_price: f64) -> f64 {
    if reserve_dusd <= 0.0 {
        log::warn!("Reserve DUSD must be positive. Received {}", reserve_dusd);
        return 0.0;
    }
    let dex_price = (reserve_dfi / reserve_dusd) * dfi_oracle_price;
    if dex_price < 0.99 {
        let coeff = *COEFF_DISCOUNT.read().unwrap();
        coeff.powf(0.99 - dex_price) - 1.0
    } else if dex_price < 1.01 {
        0.0
    } else if dex_price < 1.05 {
        let coeff = *COEFF_PREMIUM.read().unwrap();
        1.0 - coeff.powf(dex_price - 1.01)
    } else {
        -0.05
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_dex_fee, calc_loan_interest_rate};

    #[test]
    fn test_default_fees() {
        let cases = &[
            (510.0, 0.0),
            (500.0, 0.0),
            (490.0, 0.005895),
            (480.0, 0.011825),
            (450.0, 0.029825),
            (400.0, 0.06054),
            (350.0, 0.092172),
            (250.0, 0.158292),
            (100.0, 0.265054),
        ];

        for &(algo_dusd, fee) in cases {
            let f = calc_dex_fee(algo_dusd, 1000.0);
            if (fee - f).abs() > 0.000001 {
                panic!("Fee {} is off from expected fee {}", f, fee);
            }
        }
    }

    #[test]
    fn test_default_rates() {
        let cases = &[
            (1.05, -0.05),
            (1.04, -0.037395),
            (1.03, -0.024777),
            (1.02, -0.012313),
            (1.01, 0.0),
            (0.99, 0.0),
            (0.98, 0.0641178),
            (0.97, 0.1323466),
            (0.95, 0.282209),
            (0.9, 0.749473),
            (0.8, 2.2569),
            (0.7, 5.063192),
        ];

        for &(price, rate) in cases {
            let r = calc_loan_interest_rate(1.0, 1.0, price);
            if (rate - r).abs() > 0.000001 {
                panic!("Rate {} is off from expected {}", r, rate);
            }
        }
    }
}
