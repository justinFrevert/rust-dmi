use crate::dmi::{DMIError, DMI};
use ndarray::{s, Array2, ArrayView2, Axis};

struct DMIUser;
impl DMI for DMIUser {}

#[test]
pub fn calculate_factorials_works() {
    let result: Result<_, _> = <DMIUser as DMI>::calculate_factorials(&8, &4);
    assert_eq!(result, Ok(70.));
}

#[test]
fn check_works() {
    let x = 0;
    let c = 4;

    let result = <DMIUser as DMI>::check_answers(&x, &c);
    assert_eq!(result, true);
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

    let mut expected = Array2::<f32>::zeros((4, 4));

    *expected.get_mut((0, 0)).unwrap() = 4.0;
    assert_eq!(result, Ok(expected));
}

#[test]
fn get_mutual_information_works() {
    let a1 = Array2::<usize>::zeros((4, 4));
    let b1 = Array2::<usize>::zeros((4, 4));
    let a2 = Array2::<usize>::zeros((4, 4));
    let b2 = Array2::<usize>::zeros((4, 4));

    let a1 = a1.slice(s![1, ..,]);
    let b1 = b1.slice(s![1, ..,]);
    let a2 = a2.slice(s![1, ..,]);
    let b2 = b2.slice(s![1, ..,]);
    let c = 4;

    let result = <DMIUser as DMI>::get_mutual_information(a1, b1, a2, b2, &c).unwrap();

    let expected = 0.;
    assert_eq!(result, expected);
}

#[test]
fn get_mutual_information_works_2() {
    let a1 = Array2::<usize>::eye(4);
    let b1 = Array2::<usize>::eye(4);
    let a2 = Array2::<usize>::eye(4);
    let b2 = Array2::<usize>::eye(4);

    let a1 = a1.slice(s![1, ..,]);
    let b1 = b1.slice(s![1, ..,]);
    let a2 = a2.slice(s![1, ..,]);
    let b2 = b2.slice(s![1, ..,]);
    let c = 4;

    let result = <DMIUser as DMI>::get_mutual_information(a1, b1, a2, b2, &c).unwrap();

    let expected = 0.;
    assert_eq!(result, expected);
}

#[test]
fn get_mutual_information_works_3() {
    let a1 = Array2::<usize>::from_elem((4, 4), 5);
    let b1 = Array2::<usize>::eye(4);
    let a2 = Array2::<usize>::zeros((4, 4));
    let b2 = Array2::<usize>::from_elem((4, 4), 1);

    let a1 = a1.slice(s![1, ..,]);
    let b1 = b1.slice(s![3, ..,]);
    let a2 = a2.slice(s![3, ..,]);
    let b2 = b2.slice(s![2, ..,]);
    let c = 8;

    let result = <DMIUser as DMI>::get_mutual_information(a1, b1, a2, b2, &c).unwrap();

    let expected = 0.;
    assert_eq!(result, expected);
}

#[test]
fn do_dmi_works() {
    let a1 = Array2::<usize>::zeros((4, 4));

    let result = <DMIUser as DMI>::do_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 12];
    assert_eq!(expected, result)
}

#[test]
fn do_dmi_works_2() {
    let a1 = Array2::<usize>::from_elem((4, 4), 1);

    let result = <DMIUser as DMI>::do_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 12];
    assert_eq!(expected, result)
}

#[test]
fn do_dmi_works_3() {
    let a1 = Array2::<usize>::eye(5);

    let result = <DMIUser as DMI>::do_dmi(a1, 2).unwrap();

    let expected: Vec<f32> = vec![0.; 20];
    assert_eq!(expected, result)
}

#[test]
fn calculate_payments_works() {
    let agent_n = 2;
    let choice_n = 3;
    let half = agent_n / 2;

    let answers = Array2::<usize>::zeros((4, 4));
    let answers = answers.t();

    let view: ArrayView2<usize> = answers.view();
    let (a1, b1) = view.split_at(Axis(0), half);
    let a1 = a1.t();
    let b1 = b1.t();

    let result = <DMIUser as DMI>::calculate_payments(&agent_n, &choice_n, a1, b1);

    assert_eq!(result, Ok(vec![0., 0.]));
}

#[test]
fn calculate_payments_errs_with_high_choice_n() {
    let agent_n = 4;
    let choice_n = 4;
    let half = agent_n / 2;

    let answers = Array2::<usize>::zeros((4, 4));

    let transposed = answers.t();
    let view = ArrayView2::from(transposed);
    let (a1, b1) = view.split_at(Axis(0), half);

    let result = <DMIUser as DMI>::calculate_payments(&agent_n, &choice_n, a1, b1);

    assert_eq!(result, Err(DMIError::NLessThanM));
}
