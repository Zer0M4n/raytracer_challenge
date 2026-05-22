const EPSILON: f64 = 0.00001;
// This fuctions exist for comparing float number with EPSILON for round of Error make two number that should
// be  equivalent for instead be slighty different
pub fn comparing_floating_number(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}
