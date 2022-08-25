//! Ported from https://github.com/DeFiCh/ain/blob/ea0c12f843970de53b613cad11a6f2f727a59e89/src/amount.h

use ethnum::U256;

use std::ops::{Add, Div, Mul, Sub};

/// Amount in satoshis (can be negative)
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Amount(pub i64);

impl Amount {
    pub const COIN: Amount = Amount(100000000);

    pub fn checked_price_multiply(self, price: Self) -> Self {
        Amount(*U256::new((self.0 as u128 * price.0 as u128) / Self::COIN.0 as u128).low() as i64)
    }

    pub fn checked_price_divide(self, price: Self) -> Option<Self> {
        if price.0 == 0 {
            return None;
        }

        Some(Amount(
            *U256::new((self.0 as u128 * Self::COIN.0 as u128) / price.0 as u128).low() as i64,
        ))
    }

    pub fn as_raw(&self) -> f64 {
        self.0 as f64 / Self::COIN.0 as f64
    }

    pub fn from_raw(f: f64) -> Self {
        Amount((f * Self::COIN.0 as f64) as i64)
    }
}

// Ops impl
impl Add for Amount {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Amount(self.0 + other.0)
    }
}
impl Sub for Amount {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Amount(self.0 - other.0)
    }
}
impl Mul for Amount {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Amount(self.0 * other.0)
    }
}
impl Div for Amount {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Amount(self.0 / other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Amount;

    #[test]
    fn test_multiply() {
        let cases = &[(50, 50, 2500)];

        for &(a, b, r) in cases {
            let a = Amount(a);
            let b = Amount(b) * Amount::COIN;
            assert_eq!(a.checked_price_multiply(b).0, r);
        }
    }

    #[test]
    fn test_divide() {
        let cases = &[(0, 0, 0), (50, 50, 1)];

        for &(a, b, r) in cases {
            let a = Amount(a);
            let b = Amount(b) * Amount::COIN;
            if r == 0 {
                assert!(a.checked_price_divide(b).is_none());
            } else {
                let v = a.checked_price_divide(b).unwrap();
                assert_eq!(v.0, r);
            }
        }
    }
}
