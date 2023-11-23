use num::Float;

#[derive(Debug)]
pub struct Point2D<T: num::Float> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Line2D<T: num::Float> {
    pub points: [Point2D<T>; 2],
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
    pub fn length(&self) -> T {
        return (((self.points[0].x - self.points[1].x) * (self.points[0].x - self.points[1].x))
            + ((self.points[0].y - self.points[1].y) * (self.points[0].y - self.points[1].y)))
            .sqrt();
    }

    pub fn new((x1, y1): (T, T), (x2, y2): (T, T)) -> Line2D<T> {
        Line2D {
            points: [Point2D::new((x1, y1)), Point2D::new((x2, y2))],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-10;

    #[test]
    fn test_length_calculation() {
        let test_line = Line2D {
            points: [Point2D { x: 0.0, y: -0.3 }, Point2D { x: 0.1, y: 0.2 }],
        };

        assert!((test_line.length() - 0.50990194).abs() < EPSILON);
    }

    #[test]
    fn test_equal_points_have_zero_length() {
        let test_line = Line2D {
            points: [Point2D { x: 0.0, y: -0.3 }, Point2D { x: 0.0, y: -0.3 }],
        };

        assert!((test_line.length()) < EPSILON);
    }

    #[test]
    fn test_length_calculation_alternative_construction() {
        let test_line = Line2D::new((0.0, -0.3), (0.1, 0.2));

        assert!((test_line.length() - 0.50990194).abs() < EPSILON);
    }

    #[test]
    fn test_equal_points_have_zero_length_alternative_construction() {
        let test_line = Line2D::new((0.0, -0.3), (0.0, -0.3));

        assert!((test_line.length()) < EPSILON);
    }
}
