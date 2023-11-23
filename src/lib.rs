use num::Float;

mod simulation;

use simulation::view_factor_sim::EmissiveShape;

pub struct EmissiveLine<T: Float> {
    pub line: Line2D<T>,
    name: String,
    selected_normal_index: i32,
}

impl<T> EmissiveShape<T> for EmissiveLine<T>
where
    T: Float,
{
    fn get_normal(&self) -> [Point2D<T>; 2] {
        let dx = self.line.points[1].x - self.line.points[0].x;
        let dy = self.line.points[1].y - self.line.points[0].y;

        return [Point2D::new((-dy, dx)), Point2D::new((dy, -dx))];
    }

    fn set_normal(&mut self, i: u32) {
        if i != 0 && i != 1 {
            panic!("Invalid index for normal of a line.  There are only two valid solutions (index={})", i);
        }
        todo!()
    }

    fn set_name(&mut self, name: String) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct Point2D<T: num::Float> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Line2D<T: num::Float> {
    points: [Point2D<T>; 2],
    angle: T,
    length: T,
}

impl<T> Point2D<T>
where
    T: Float,
{
    pub fn new((x1, y1): (T, T)) -> Point2D<T> {
        Point2D { x: x1, y: y1 }
    }
}

impl<T> Line2D<T>
where
    T: Float,
{
    pub fn get_angle(&self) -> T {
        return self.angle;
    }

    pub fn get_length(&self) -> T {
        return self.length;
    }

    pub fn new((x1, y1): (T, T), (x2, y2): (T, T)) -> Line2D<T> {
        Line2D {
            points: [Point2D::new((x1, y1)), Point2D::new((x2, y2))],
            angle: T::atan2(y2 - y1, x2 - x1).to_degrees(),
            length: (((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2))).sqrt(),
        }
    }

    // TODO: Implement this
    // pub fn new_with_r_theta((x1, y1): (T, T), theta: T, r: T) -> Line2D<T> {
    //     Line2D {
    //         points: (),
    //         angle: theta,
    //         length: r,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-10;

    #[test]
    fn test_length_calculation_alternative_construction() {
        let test_line = Line2D::new((0.0, -0.3), (0.1, 0.2));

        assert_eq!(test_line.get_angle(), 78.69007);
        assert!((test_line.get_length() - 0.50990194).abs() < EPSILON);
    }

    #[test]
    fn test_equal_points_have_zero_length_alternative_construction() {
        let test_line = Line2D::new((0.0, -0.3), (0.0, -0.3));

        assert_eq!(test_line.get_angle(), 0.0);
        assert!((test_line.get_length()) < EPSILON);
    }

    #[test]
    fn test_angle_of_straight_up_line() {
        let test_line = Line2D::new((0.0, 0.0), (0.0, 2.0));

        assert_eq!(test_line.get_angle(), 90.0);
    }

    #[test]
    fn test_angle_of_straight_right_line() {
        let test_line = Line2D::new((0.0, 0.0), (2.0, 0.0));

        assert_eq!(test_line.get_angle(), 0.0);
    }
}
