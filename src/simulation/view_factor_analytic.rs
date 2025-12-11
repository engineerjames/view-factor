pub fn unequal_normal_adjacent_strips(height: f64, width: f64) -> f64 {
    let h = height / width;

    (1.0 + h - f64::sqrt(1.0 + (h * h))) / 2.0
}

#[allow(dead_code)]
pub fn parallel_strips(separation: f64, width_1: f64, width_2: f64) -> f64 {
    let w1 = width_1 / separation;
    let w2 = width_2 / separation;

    let denom = 2.0 * w1;
    let num1 = f64::sqrt(((w1 + w2) * (w1 + w2)) + 4.0);
    let num2 = f64::sqrt(((w2 - w1) * (w2 - w1)) + 4.0);

    num1 / denom - num2 / denom
}

mod tests {
    #[test]
    fn check_unequal_normal_adjacent_strips_calculation() {
        use super::unequal_normal_adjacent_strips;

        let expected_result = 0.29289321881345243;
        let actual_result = unequal_normal_adjacent_strips(1.0, 1.0);

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn check_parallel_strips_calculation() {
        use super::parallel_strips;
        
        let expected_result = 0.41421356237309515;
        let actual_result = parallel_strips(1.0, 1.0, 1.0);
        assert_eq!(expected_result, actual_result);
    }
}
