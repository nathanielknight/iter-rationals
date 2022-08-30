//! An implementation of a fixed-size iterator over the rational numbers based on the
//! algorithm described by Gibbons, Lester, and Bird in
//! [Functional Pearl: Enumerating the Rationals].
//!
//! [Functional Pearl: Enumerating the Rationals]: http://www.cs.ox.ac.uk/people/jeremy.gibbons/publications/rationals.pdf
//!
//!

use num_integer::Integer;
use num_rational::Ratio;
use num_traits::cast::FromPrimitive;

pub struct Rationals<T: Integer + Clone> {
    state: Ratio<T>,
}

impl<T> Rationals<T>
where
    T: Integer + Clone + FromPrimitive,
{
    pub fn new() -> Self {
        Self {
            state: Self::one(),
        }
    }

    fn one() -> Ratio<T> {
        Ratio::new(
                T::from_u64(1).expect("Failed to convert '1'"),
                T::from_u64(1).expect("Failed to convert from '1'")
                )
    }
}

impl<T> Iterator for Rationals<T>
where
    T: Integer + Clone + core::ops::Add,
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.state.clone();
        let n = r.trunc();
        let y = r.fract();
        let next = (n + T::one() - y).recip();
        self.state = next;
        Some(r)
    }
}

#[test]
fn test_speed_is_reasonable() {
    let mut r = Rationals::<u32>::new();
    let millionth_rational = r.nth(1_000_000).unwrap();
    let expected = Ratio::new(1287, 1096);
    assert_eq!(expected, millionth_rational);
}

#[test]
fn test_first_values_are_as_expected() {
    let expected_parts = [
        (1, 1),
        (1, 2), (2, 1),
        (1, 3), (3, 2), (2, 3), (3, 1),
        (1, 4), (4, 3), (3, 5), (5, 2), (2, 5), (5, 3), (3, 4), (4, 1)
    ];
    let expected: Vec<Ratio<u32>> = expected_parts.iter().map(|p| Ratio::new(p.0, p.1)).collect();
    let found: Vec<Ratio<u32>> = Rationals::<u32>::new().take(expected.len()).collect();
    assert_eq!(found, expected);

}
