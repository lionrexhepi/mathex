pub mod math;

#[cfg(test)]
mod test {

    use super::math::terms::*;

    use Term::*;
    #[test]
    fn test_add() {
        let term = Addition(Box::new(Value((0.5).into())), Box::new(Value((1.2).into())));

        let value = term.get_value().unwrap();

        assert_eq!(value, Number::Rational(Fraction::new(170, 100)))
    }
}
