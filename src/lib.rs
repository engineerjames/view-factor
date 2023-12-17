
// impl<T> Line2D<T>
// where
//     T: Float,
// {
//     pub fn get_angle(&self) -> T {
//         return self.angle;
//     }

//     pub fn get_length(&self) -> T {
//         return self.length;
//     }

//     pub fn new((x1, y1): (T, T), (x2, y2): (T, T)) -> Line2D<T> {
//         Line2D {
//             points: [Point2D::new((x1, y1)), Point2D::new((x2, y2))],
//             angle: T::atan2(y2 - y1, x2 - x1).to_degrees(),
//             length: (((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2))).sqrt(),
//         }
//     }

    // TODO: Implement this
    // pub fn new_with_r_theta((x1, y1): (T, T), theta: T, r: T) -> Line2D<T> {
    //     Line2D {
    //         points: (),
    //         angle: theta,
    //         length: r,
    //     }
    // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EPSILON: f32 = 1e-10;

//     #[test]
//     fn test_length_calculation_alternative_construction() {
//         let test_line = Line2D::new((0.0, -0.3), (0.1, 0.2));

//         assert_eq!(test_line.get_angle(), 78.69007);
//         assert!((test_line.get_length() - 0.50990194).abs() < EPSILON);
//     }

//     #[test]
//     fn test_equal_points_have_zero_length_alternative_construction() {
//         let test_line = Line2D::new((0.0, -0.3), (0.0, -0.3));

//         assert_eq!(test_line.get_angle(), 0.0);
//         assert!((test_line.get_length()) < EPSILON);
//     }

//     #[test]
//     fn test_angle_of_straight_up_line() {
//         let test_line = Line2D::new((0.0, 0.0), (0.0, 2.0));

//         assert_eq!(test_line.get_angle(), 90.0);
//     }

//     #[test]
//     fn test_angle_of_straight_right_line() {
//         let test_line = Line2D::new((0.0, 0.0), (2.0, 0.0));

//         assert_eq!(test_line.get_angle(), 0.0);
//     }
// }
