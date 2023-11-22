#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Line {
    pub points: [Point; 2],
}

impl Point {
    pub fn new((x1, y1): (f32, f32)) -> Point {
        Point { x: x1, y: y1 }
    }
}

impl Line {
    pub fn length(&self) -> f32 {
        return ((self.points[0].x - self.points[1].x).powf(2.0)
            + (self.points[0].y - self.points[1].y).powf(2.0))
        .sqrt();
    }

    pub fn new((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> Line {
        Line {
            points: [Point::new((x1, y1)), Point::new((x2, y2))],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-10;

    #[test]
    fn test_length_calculation() {
        let test_line = Line {
            points: [Point { x: 0.0, y: -0.3 }, Point { x: 0.1, y: 0.2 }],
        };

        assert!((test_line.length() - 0.50990194).abs() < EPSILON);
    }

    #[test]
    fn test_equal_points_have_zero_length() {
        let test_line = Line {
            points: [Point { x: 0.0, y: -0.3 }, Point { x: 0.0, y: -0.3 }],
        };

        assert!((test_line.length()) < EPSILON);
    }

    #[test]
    fn test_length_calculation_alternative_construction() {
        let test_line = Line::new((0.0, -0.3), (0.1, 0.2));

        assert!((test_line.length() - 0.50990194).abs() < EPSILON);
    }

    #[test]
    fn test_equal_points_have_zero_length_alternative_construction() {
        let test_line = Line::new((0.0, -0.3), (0.0, -0.3));

        assert!((test_line.length()) < EPSILON);
    }
}
