mod dmi;

use dmi::DMI;

use ndarray::prelude::*;
use ndarray_linalg::{error::LinalgError, solve::Determinant};

struct DMIData;

impl dmi::DMI for DMIData {}

fn main() {
    let testing_c = 4;
    let mut answers = Array2::<usize>::zeros((testing_c, testing_c));

    println!("the answers before calculate dmi are {:?}", answers);

    let calculated = <crate::DMIData as DMI>::calculate_dmi(answers, 2);

    // dmi::DMI::calculate_dmi(answers);

    // dmi.calculate_dmi(answers);

    println!("Hello, world! {:?}", calculated);
}
