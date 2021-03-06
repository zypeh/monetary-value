use crate::num_rational::*;

#[derive(Debug)]
pub struct MonetaryValue<T> {
    value: T,
    precision: u8,
}

#[derive(Debug)]
// This num-rational library only support i32 and i64
pub struct LosslessMonetaryValue<T> {
    value: Ratio<T>,
    precision: u8,
}

pub trait Money {
    fn to_money_decimal(self, p: u8) -> MonetaryValue<i64>;
}

impl Money for i64 {
    fn to_money_decimal(self, p: u8) -> MonetaryValue<i64> {
        MonetaryValue {
            value: self,
            precision: p,
        }
    }
}

pub fn what(s: MonetaryValue<i64>) {
    println!("{:?}", s);
}
