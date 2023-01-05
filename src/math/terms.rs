use std::ops::{Add, Mul};

use fraction::{FromPrimitive, Ratio, ToPrimitive};

pub type Fraction = Ratio<i64>;

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Rational(Fraction),
    Irrational(f64),
}

impl Number {
    fn pow_frac(frac1: &Fraction, frac2: &Fraction) -> Number {
        if *frac2.denom() == 1i64 {
            let power = i32::from_i64(*frac2.numer());

            if let Some(p) = power {
                return Self::Rational(frac1.pow(p));
            } else {
                panic!("{} is too powerful!", frac2.denom())
            }
        } else {
            return f64::powf(frac1.to_f64().unwrap(), frac2.to_f64().unwrap()).into();
        }
    }

    pub fn pow(&self, other: &Number) -> Number {
        match self {
            Self::Rational(frac) => match other {
                Self::Rational(other_frac) => Self::pow_frac(frac, other_frac),
                Self::Irrational(value) => f64::powf(frac.to_f64().unwrap(), *value).into(),
            },
            Self::Irrational(value) => (*value).powf(Into::<f64>::into(*other)).into(),
        }
    }

    fn inverse(self) -> Self {
        match self {
            Number::Rational(frac) => Self::Rational(frac.recip()),
            Number::Irrational(v) => Self::Irrational(1.0 / v),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rational(l0), Self::Rational(r0)) => l0 == r0,
            (Self::Irrational(l0), Self::Irrational(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Number::Rational(fraction) => match rhs {
                Number::Rational(other) => Number::Rational(other * fraction),
                Number::Irrational(value) => f64::mul(self.into(), value).into(),
            },
            Number::Irrational(value) => Mul::<f64>::mul(value, rhs.into()).into(),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Number::Rational(fraction) => match rhs {
                Number::Rational(other) => Number::Rational(other + fraction),
                Number::Irrational(value) => f64::add(self.into(), value).into(),
            },
            Number::Irrational(value) => Add::<f64>::add(value, rhs.into()).into(),
        }
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        match value {
            Number::Rational(fraction) => fraction.to_f64().unwrap(),
            Number::Irrational(v) => v,
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        if let Some(fraction) = Fraction::from_f64(value) {
            Self::Rational(fraction)
        } else {
            Self::Irrational(value)
        }
    }
}

pub enum Term {
    Value(Number),
    Variable(Box<str>),
    Addition(Box<Term>, Box<Term>),
    Multiplication(Box<Term>, Box<Term>),
    Exponentation(Box<Term>, Box<Term>),
    RootExtraction(Box<Term>, Box<Term>),
}

use Term::*;

impl Term {
    pub fn has_value(&self) -> bool {
        match self {
            Value(_) => true,
            Variable(_) => false,
            Addition(lhs, rhs) => lhs.has_value() && rhs.has_value(),
            Multiplication(lhs, rhs) => lhs.has_value() && rhs.has_value(),
            Exponentation(base, power) => base.has_value() && power.has_value(),
            RootExtraction(radicand, degree) => radicand.has_value() && degree.has_value(),
        }
    }

    pub fn substitute(self, name: &str, value: Number) -> Self {
        if let Variable(var) = &self {
            if str::eq(&*var, name) {
                Value(value)
            } else {
                self
            }
        } else {
            match self {
                Addition(lhs, rhs) => {
                    let (lhs, rhs) = (
                        lhs.substitute(name, value.clone()),
                        rhs.substitute(name, value),
                    );

                    let (lhs, rhs) = (Box::new(lhs), Box::new(rhs));

                    Addition(lhs, rhs)
                }
                Multiplication(lhs, rhs) => {
                    let (lhs, rhs) = (
                        lhs.substitute(name, value.clone()),
                        rhs.substitute(name, value),
                    );

                    let (lhs, rhs) = (Box::new(lhs), Box::new(rhs));

                    Multiplication(lhs, rhs)
                }
                Exponentation(base, power) => {
                    let (base, power) = (
                        base.substitute(name, value.clone()),
                        power.substitute(name, value),
                    );

                    let (base, power) = (Box::new(base), Box::new(power));

                    Exponentation(base, power)
                }

                RootExtraction(radicand, degree) => {
                    let (radicand, degree) = (
                        radicand.substitute(name, value.clone()),
                        degree.substitute(name, value),
                    );

                    let (radicand, degree) = (Box::new(radicand), Box::new(degree));

                    RootExtraction(radicand, degree)
                }

                _ => self,
            }
        }
    }

    pub fn get_value(&self) -> Option<Number> {
        if !self.has_value() {
            None
        } else {
            Some(match self {
                Value(v) => v.clone(),
                Addition(lhs, rhs) => lhs.get_value().unwrap() + rhs.get_value().unwrap(),
                Multiplication(lhs, rhs) => lhs.get_value().unwrap() + rhs.get_value().unwrap(),
                Exponentation(base, power) => {
                    base.get_value().unwrap().pow(&power.get_value().unwrap())
                }
                RootExtraction(radicand, degree) => radicand
                    .get_value()
                    .unwrap()
                    .pow(&degree.get_value().unwrap().inverse()),
                Variable(_) => panic!("How did we get here? Variables don't have values."),
            })
        }
    }
}
