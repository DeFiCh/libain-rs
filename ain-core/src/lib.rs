mod coin;

use self::coin::Amount;

use std::error::Error;
use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref COEFF_DEX_FEE: RwLock<Amount> = RwLock::new(Amount(180000000));
    static ref COEFF_DISCOUNT: RwLock<Amount> = RwLock::new(Amount(50000000000));
    static ref COEFF_PREMIUM: RwLock<Amount> = RwLock::new(Amount(340000000));
}

const DUSD_PRICE_FLOOR: Amount = Amount(99000000);
const DUSD_PRICE_CEIL: Amount = Amount(105000000);
const DUSD_PRICE_IDEAL: Amount = Amount(101000000);
const RATIO_HALF: Amount = Amount(Amount::COIN.0 / 2);

#[cxx::bridge]
mod calc {
    extern "Rust" {
        fn set_fee_coefficient(coefficient: i64);

        fn set_interest_rate_coefficients(discount: i64, premium: i64);

        fn calc_dex_fee(algo_dusd: i64, dusd_supply: i64) -> Result<i64>;

        fn calc_loan_interest_rate(
            reserve_dfi: i64,
            reserve_dusd: i64,
            dfi_oracle_price: i64,
        ) -> Result<i64>;
    }
}

/// **DFIP-2206-D**
///
/// Sets the coefficient for calculation of dex stabilization fee. Default is `1.8`
fn set_fee_coefficient(coefficient: i64) {
    *COEFF_DEX_FEE.write().unwrap() = Amount(coefficient);
}

/// **DFIP-2206-E**
///
/// Sets the coefficients for discount and premium of DUSD for calculation of interest rates.
/// Default is `500` for discount and `3.4` for premium.
fn set_interest_rate_coefficients(discount: i64, premium: i64) {
    *COEFF_DISCOUNT.write().unwrap() = Amount(discount);
    *COEFF_PREMIUM.write().unwrap() = Amount(premium);
}

/// **DFIP-2206-D**
///
/// Calculates the dynamic dex stabilization fee for DUSD on DUSD-DFI pair. The size of the fee
/// is determined by the ratio of algorithmic DUSD to the total amount of outstanding DUSD.
fn calc_dex_fee(algo_dusd: i64, dusd_supply: i64) -> Result<i64, Box<dyn Error>> {
    let ratio = Amount::COIN
        - Amount(algo_dusd)
            .checked_price_divide(Amount(dusd_supply))
            .ok_or("Cannot divide given DUSD supply")?;

    let coeff = COEFF_DEX_FEE.read().unwrap().as_raw();
    if ratio > RATIO_HALF {
        Ok(Amount::from_raw(coeff.powf((ratio - RATIO_HALF).as_raw()) - 1.0).0)
    } else {
        Ok(0)
    }
}

/// **DFIP-2206-E**
///
/// Calculates the dynamic interest rates on DUSD loans, based on the current discount/premium
/// of DUSD evaluated with the DFI price oracle.
fn calc_loan_interest_rate(
    reserve_dfi: i64,
    reserve_dusd: i64,
    dfi_oracle_price: i64,
) -> Result<i64, Box<dyn Error>> {
    if reserve_dfi <= 0 || reserve_dusd <= 0 {
        return Err("Reserve token amount must be positive".into());
    }

    let price = Amount(reserve_dfi)
        .checked_price_divide(Amount(reserve_dusd))
        .map(|a| a.checked_price_multiply(Amount(dfi_oracle_price)))
        .ok_or("Token amount is out of bounds")?;

    if price < DUSD_PRICE_FLOOR {
        let coeff = COEFF_DISCOUNT.read().unwrap().as_raw();
        Ok(Amount::from_raw(coeff.powf((DUSD_PRICE_FLOOR - price).as_raw()) - 1.0).0)
    } else if price < DUSD_PRICE_IDEAL {
        Ok(0)
    } else if price < DUSD_PRICE_CEIL {
        let coeff = COEFF_PREMIUM.read().unwrap().as_raw();
        Ok(Amount::from_raw(1.0 - coeff.powf((price - DUSD_PRICE_IDEAL).as_raw())).0)
    } else {
        Ok(-5000000)
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_dex_fee, calc_loan_interest_rate};

    #[test]
    fn test_default_fees() {
        let cases = &[
            (510, 0),
            (500, 0),
            (490, 589517),
            (480, 1182510),
            (450, 2982546),
            (400, 6054048),
            (350, 9217159),
            (250, 15829218),
            (100, 26505381),
        ];

        for &(algo_dusd, fee) in cases {
            let f = calc_dex_fee(algo_dusd, 1000).unwrap();
            assert_eq!(f, fee);
        }
    }

    #[test]
    fn test_default_rates() {
        let cases = &[
            (105000000, -5000000),
            (104000000, -3739551),
            (103000000, -2477749),
            (102000000, -1231294),
            (101000000, 0),
            (99000000, 0),
            (98000000, 6411778),
            (97000000, 13234665),
            (95000000, 28220893),
            (90000000, 74947322),
            (80000000, 225689907),
            (70000000, 506319172),
        ];

        for &(price, rate) in cases {
            let r = calc_loan_interest_rate(1, 1, price).unwrap();
            assert_eq!(r, rate);
        }
    }
}
