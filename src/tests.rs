use crate::dmi::DMI;
use ndarray::{s, Array2, ArrayView2, Axis};
use ndarray_linalg::generate::random;

struct DMIUser;
impl DMI for DMIUser {}

#[test]
pub fn comb_works() {
    // let result = TestDmi::comb(&8, &4);
    let result: f32 = <DMIUser as DMI>::comb(&8, &4);
    assert_eq!(result, 70.);
}

#[test]
fn get_mechanism_works() {
    let a1 = Array2::<usize>::zeros((4, 4));
    let b1 = Array2::<usize>::zeros((4, 4));

    // Get two slices... first rows of each only
    let a = a1.slice(s![1, ..,]);
    let b = b1.slice(s![1, ..,]);
    let c = 4;

    let result = <DMIUser as DMI>::get_mechanism(a, b, &c);

    // TODO: Validate python expected behavior
    let mut expected = Array2::<f32>::zeros((4, 4));
    *expected.get_mut((0, 0)).unwrap() = 4.0;
    assert_eq!(result, expected);
    // Remove after validating python behavior
    // assert!(false);
}

#[test]
fn dmi_inner_works() {
    let a1 = Array2::<usize>::zeros((4, 4));
    let b1 = Array2::<usize>::zeros((4, 4));
    let a2 = Array2::<usize>::zeros((4, 4));
    let b2 = Array2::<usize>::zeros((4, 4));

    let a1 = a1.slice(s![1, ..,]);
    let b1 = b1.slice(s![1, ..,]);
    let a2 = a2.slice(s![1, ..,]);
    let b2 = b2.slice(s![1, ..,]);
    let c = 4;

    let result = <DMIUser as DMI>::dmi_inner(a1, b1, a2, b2, &c).unwrap();

    // TODO: Validate python expected behavior
    let expected = 0.;
    assert_eq!(result, expected);
    // Remove after validating python behavior
    // assert!(false);
}

#[test]
fn calculate_dmi_works() {
    let a1 = Array2::<usize>::zeros((4, 4));

    let result = <DMIUser as DMI>::calculate_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 12];
    assert_eq!(expected, result)
    // Remove after validating python behavior
    // assert!(false);
}

#[test]
fn calculate_payments_works() {
    let agent_n = 4;
    let choice_n = 4;
    let half = agent_n / 2;

    let answers = Array2::<usize>::zeros((4, 4));

    let transposed = answers.t();
    let view = ArrayView2::from(transposed);
    let (a1, b1) = view.split_at(Axis(0), half);

    let result = <DMIUser as DMI>::calculate_payments(&agent_n, &choice_n, a1, b1);
}

#[test]
fn gets_dmi_inner() {
    let a1 = Array2::<usize>::zeros((1, 0));
    let b1 = Array2::<usize>::zeros((1, 0));
    let a2 = Array2::<usize>::zeros((1, 0));
    let b2 = Array2::<usize>::zeros((1, 0));

    // let result = TestDmi::dmi_inner(a1, b1, a2, b2, 1);

    // assert_eq!(result, 42.);
}
