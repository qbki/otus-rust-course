use super::eq_floats;

#[test]
fn values_should_be_equal() {
    assert!(eq_floats(0.0, 0.0));
    assert!(eq_floats(-1.0, -1.0));
    assert!(eq_floats(1.0, 1.0));
}

#[test]
fn values_should_not_be_equal() {
    assert!(!eq_floats(1.0, -1.0));
    assert!(!eq_floats(-1.0, 1.0));
}
