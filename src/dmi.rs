use std::ops::DerefMut;

use factorial::Factorial;
use ndarray::{aview_mut2, prelude::*};
use ndarray_linalg::{error::LinalgError, solve::Determinant, DeterminantInto};

enum DMIError {
    /// The values of answers must be integers in [0, C)
    AnswerValsOutOfScope,
}

trait DMI {
    fn comb(n: &usize, m: &usize) -> usize {
        // TODO: Obv handle unwrapped cases later..
        Factorial::checked_factorial(n).unwrap()
            / (Factorial::checked_factorial(m).unwrap()
                * Factorial::checked_factorial(&(n - m)).unwrap())
    }

    // In source it asks that it should be i64... consider changing later.
    fn check(x: &usize, c: &usize) -> bool {
        &0 <= x && x < c
    }

    // get "M"
    // a and b are equal length
    fn get_mechanism<'a>(a: Array2<usize>, b: Array2<usize>, c: usize) -> Array2<f32> {
        let mut mechanism = Array2::<usize>::zeros((c, c));
        for (x, y) in a.into_iter().zip(b.into_iter()) {
            if Self::check(&x, &c) && Self::check(&y, &c) {
                *mechanism.get_mut((c, c)).unwrap() += 1;
            } else {
                println!("Error: AnswerValsOutOfScope");
                // Err(DMIError::AnswerValsOutOfScope)
            }
        }
        // Return it back to some floating point due to requirements in determinant calc... requires a few other changes
        mechanism.map(|k| *k as f32)
    }

    // aka dmi2 in source
    fn dmi_inner(
        a1: Array2<usize>,
        b1: Array2<usize>,
        a2: Array2<usize>,
        b2: Array2<usize>,
        c: usize,
    ) -> Result<f32, LinalgError> {
        let m1 = Self::get_mechanism(a1, b1, c);
        let m2 = Self::get_mechanism(a2, b2, c);
        Ok(m1.det()? * m2.det()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::dmi::DMI;
    use factorial::Factorial;

    struct TestDmi;
    impl DMI for TestDmi {
        fn comb(n: &usize, m: &usize) -> usize {
            // TODO: Obv handle unwrapped cases later..
            Factorial::checked_factorial(n).unwrap()
                / (Factorial::checked_factorial(m).unwrap()
                    * Factorial::checked_factorial(&(n - m)).unwrap())
        }
    }

    #[test]
    pub fn dmi_comb() {
        let result = TestDmi::comb(&8, &4);
        assert_eq!(result, 70);
    }

    pub fn dmi_determinant() {
        let result = TestDmi::dmi2(a1, b1, a2, b2, c);
        assert_eq!(result, 70);
    }

    fn gets_dmi() {

    }
}
