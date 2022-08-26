use crate::dmi::{DMIError, DMI};
use ndarray::{s, Array2, ArrayView2, Axis};
use ndarray_linalg::generate::random;

struct DMIUser;
impl DMI for DMIUser {}

// #[test]
// // ✅ Currently works equally to python function + test
// pub fn comb_works() {
//     // let result = TestDmi::comb(&8, &4);
//     let result: Result<_, _> = <DMIUser as DMI>::comb(&8, &4);
//     assert_eq!(result, Ok(70.));
// }

// #[test]
// // ✅ Is matching python test/app behavior
// fn check_works() {
//     let x = 0;
//     let y = 0;
//     let c = 4;

//     // let result =
//     let result = <DMIUser as DMI>::check(&x, &c);
//     assert_eq!(result, true);
// }

// #[test]
// // ✅ Is matching python test/app behavior
// fn get_mechanism_works() {
//     let a1 = Array2::<usize>::zeros((4, 4));
//     let b1 = Array2::<usize>::zeros((4, 4));

//     // Get two slices... first rows of each only
//     let a = a1.slice(s![1, ..,]);
//     let b = b1.slice(s![1, ..,]);
//     let c = 4;

//     let result = <DMIUser as DMI>::get_mechanism(a, b, &c);

//     // TODO: Validate python expected behavior
//     let mut expected = Array2::<f32>::zeros((4, 4));

//     *expected.get_mut((0, 0)).unwrap() = 4.0;
//     assert_eq!(result, Ok(expected));
// }

// #[test]
// fn dmi_inner_works() {
//     let a1 = Array2::<usize>::zeros((4, 4));
//     let b1 = Array2::<usize>::zeros((4, 4));
//     let a2 = Array2::<usize>::zeros((4, 4));
//     let b2 = Array2::<usize>::zeros((4, 4));

//     let a1 = a1.slice(s![1, ..,]);
//     let b1 = b1.slice(s![1, ..,]);
//     let a2 = a2.slice(s![1, ..,]);
//     let b2 = b2.slice(s![1, ..,]);
//     let c = 4;

//     let result = <DMIUser as DMI>::dmi_inner(a1, b1, a2, b2, &c).unwrap();

//     let expected = 0.;
//     assert_eq!(result, expected);
// }

// #[test]
// fn dmi_inner_works_2() {
//     let a1 = Array2::<usize>::eye(4);
//     let b1 = Array2::<usize>::eye(4);
//     let a2 = Array2::<usize>::eye(4);
//     let b2 = Array2::<usize>::eye(4);

//     let a1 = a1.slice(s![1, ..,]);
//     let b1 = b1.slice(s![1, ..,]);
//     let a2 = a2.slice(s![1, ..,]);
//     let b2 = b2.slice(s![1, ..,]);
//     let c = 4;

//     let result = <DMIUser as DMI>::dmi_inner(a1, b1, a2, b2, &c).unwrap();

//     let expected = 0.;
//     assert_eq!(result, expected);
// }

// #[test]
// // ✅ Matches python source behavior
// fn dmi_inner_works_3() {
//     // Array::from_elem((3, 4), 7.)
//     let a1 = Array2::<usize>::from_elem((4, 4), 5);
//     let b1 = Array2::<usize>::eye(4);
//     let a2 = Array2::<usize>::zeros((4, 4));
//     let b2 = Array2::<usize>::from_elem((4, 4), 1);

//     let a1 = a1.slice(s![1, ..,]);
//     let b1 = b1.slice(s![3, ..,]);
//     let a2 = a2.slice(s![3, ..,]);
//     let b2 = b2.slice(s![2, ..,]);
//     let c = 8;

//     let result = <DMIUser as DMI>::dmi_inner(a1, b1, a2, b2, &c).unwrap();

//     let expected = 0.;
//     assert_eq!(result, expected);
// }

#[test]
// ✅ Matches with python source behavior
fn calculate_dmi_works() {
    let a1 = Array2::<usize>::zeros((4, 4));

    let result = <DMIUser as DMI>::calculate_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 12];
    assert_eq!(expected, result)
}

#[test]
// ✅ Matches with python source behavior
fn calculate_dmi_works_2() {
    let a1 = Array2::<usize>::from_elem((4, 4), 1);

    let result = <DMIUser as DMI>::calculate_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 12];
    assert_eq!(expected, result)
}

#[test]
// ✅ Matches with python source behavior
fn calculate_dmi_works_3() {
    let a1 = Array2::<usize>::eye(5);

    let result = <DMIUser as DMI>::calculate_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 20];
    assert_eq!(expected, result)
}

#[test]
fn calculate_payments_works() {
    let agent_n = 2;
    let choice_n = 3;
    let half = agent_n / 2;

    let answers = Array2::<usize>::zeros((4, 4));

    let transposed = answers.t();
    let view = ArrayView2::from(transposed);
    let (a1, b1) = view.split_at(Axis(0), half);

    let result = <DMIUser as DMI>::calculate_payments(&agent_n, &choice_n, a1, b1);

    // Need to find good values to set - this test is NOT working
    assert_eq!(result, Ok(vec![0.]));
}

// #[test]
// fn calculate_payments_errs_with_high_choice_n() {
//     let agent_n = 4;
//     let choice_n = 4;
//     let half = agent_n / 2;

//     let answers = Array2::<usize>::zeros((4, 4));

//     let transposed = answers.t();
//     let view = ArrayView2::from(transposed);
//     let (a1, b1) = view.split_at(Axis(0), half);

//     let result = <DMIUser as DMI>::calculate_payments(&agent_n, &choice_n, a1, b1);

//     assert_eq!(result, Err(DMIError::NLessThanM));
// }

// #[test]
// fn gets_dmi_inner() {
//     let a1 = Array2::<usize>::zeros((1, 0));
//     let b1 = Array2::<usize>::zeros((1, 0));
//     let a2 = Array2::<usize>::zeros((1, 0));
//     let b2 = Array2::<usize>::zeros((1, 0));

//     let a1 = a1.slice(s![1, ..,]);
//     let b1 = b1.slice(s![1, ..,]);
//     let a2 = a2.slice(s![1, ..,]);
//     let b2 = b2.slice(s![1, ..,]);
//     let c = 4;

//     // let result = TestDmi::dmi_inner(a1, b1, a2, b2, 1);
//     let result = <DMIUser as DMI>::dmi_inner(a1, b1, a2, b2, &c).unwrap();
//     assert_eq!(result, 42.);
// }
