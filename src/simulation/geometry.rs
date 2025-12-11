pub type FloatType = f32;

pub const EPSILON: FloatType = FloatType::EPSILON * 4.0;

#[derive(Debug, PartialEq, Clone)]
pub struct Point2D {
    pub x: FloatType,
    pub y: FloatType,
}

impl Point2D {
    pub fn new((x1, y1): (FloatType, FloatType)) -> Point2D {
        Point2D { x: x1, y: y1 }
    }

    pub fn magnitude(&self) -> FloatType {
        FloatType::sqrt((self.x * self.x) + (self.y * self.y))
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D::new((self.x + rhs.x, self.y + rhs.y))
    }
}

impl std::ops::Add for &Point2D {
    type Output = Point2D;

    fn add(self, other: &Point2D) -> Self::Output {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for &Point2D {
    type Output = Point2D;

    fn sub(self, rhs: &Point2D) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D::new((self.x - rhs.x, self.y - rhs.y))
    }
}

pub struct Line2DState {
    pub normals: [Point2D; 2],
    pub points: [Point2D; 2],
    pub midpoint: Point2D,
    pub slope: FloatType,
    pub y_intercept: FloatType,
}

impl Line2DState {
    pub fn new(point1: Point2D, point2: Point2D) -> Line2DState {
        let dy = point2.y - point1.y;
        let dx = point2.x - point1.x;

        let midpoint = Point2D {
            x: (point1.x + point2.x) / 2.0,
            y: (point1.y + point2.y) / 2.0,
        };

        let normal_1 = Point2D::new((-dy, dx));
        let normal_2 = Point2D::new((dy, -dx));

        let normal_1_normalized =
            Point2D::new((-dy / normal_1.magnitude(), dx / normal_1.magnitude()));
        let normal_2_normalized =
            Point2D::new((dy / normal_2.magnitude(), -dx / normal_2.magnitude()));

        // TODO: Should the slope be an Option<f32>? Straight up and down lines?
        let mut slope = 0.0;
        if FloatType::abs(dx) >= EPSILON {
            slope = dy / dx;
        }

        let mut y_intercept = 0.0;

        if FloatType::abs(slope) > EPSILON {
            y_intercept = point1.y - slope * point1.x;
        }

        Line2DState {
            normals: [normal_1_normalized, normal_2_normalized],
            points: [point1, point2],
            midpoint: midpoint,
            slope: slope,
            y_intercept: y_intercept,
        }
    }
}

pub fn is_point_on_line(p: &Point2D, line: &Line2DState) -> bool {
    let result;
    if FloatType::abs(line.slope) > EPSILON {
        result = p.y - (line.slope * p.x + line.y_intercept);
    } else {
        // If we have a straight vertical or horizontal line, we just need
        // to ensure that the new point either has the same X value as BOTH points
        // that make the line, or the same Y value as BOTH points that make the line.
        return (p.x == line.points[0].x && p.x == line.points[1].x)
            || (p.y == line.points[0].y && p.y == line.points[1].y);
    }

    FloatType::abs(result) <= EPSILON
}

pub enum ShapeType {
    Line2D(Line2DState), // Just one shape for now
}

pub struct Ray {
    pub point: Point2D,
    pub angle: FloatType,
}

impl Ray {
    pub fn new(p: Point2D, angle: FloatType) -> Ray {
        Ray {
            point: p,
            angle: angle,
        }
    }
}

pub struct EmissiveShape {
    pub name: String,
    pub shape_type: ShapeType,
    outward_normal_index: usize, // 0 or 1, indicates which normal in the array is outward-facing
}

impl EmissiveShape {
    // Generic constructor?
    pub fn new(name: String, shape_type: ShapeType) -> EmissiveShape {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            outward_normal_index: 0, // Will be set by calculate_inward_normals
        }
    }

    pub fn set_inward_normal(&mut self, center: &Point2D) {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => {
                // Vector from line midpoint to center
                let to_center = Point2D {
                    x: center.x - line_state.midpoint.x,
                    y: center.y - line_state.midpoint.y,
                };

                // Dot product with each normal to see which points toward center
                let dot_0 = line_state.normals[0].x * to_center.x + line_state.normals[0].y * to_center.y;
                let dot_1 = line_state.normals[1].x * to_center.x + line_state.normals[1].y * to_center.y;

                // The normal with positive dot product points toward the center
                self.outward_normal_index = if dot_0 > dot_1 { 0 } else { 1 };
            }
        }
    }

    pub fn get_emissive_ray(&self, std_rng: &mut rand::rngs::StdRng) -> Ray {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => {
                use rand::Rng;
                let percent_along = std_rng.gen_range(0.0..1.0);

                let new_x = percent_along * line_state.points[1].x
                    + (1.0 - percent_along) * line_state.points[0].x;
                let new_y = percent_along * line_state.points[1].y
                    + (1.0 - percent_along) * line_state.points[0].y;

                let new_point = Point2D::new((new_x, new_y));

                // Add function to check if point is on the line
                if !is_point_on_line(&new_point, line_state) {
                    println!("ERROR: Invalid point on line!");
                    std::process::exit(-1);
                }

                // Use the outward-facing normal for this shape
                let source_shape_normal = &line_state.normals[self.outward_normal_index];

                // Get the base angle of the normal
                let normal_angle_deg =
                    FloatType::atan2(source_shape_normal.y, source_shape_normal.x).to_degrees();

                // Sample angle with cosine-weighted distribution (Lambert's Cosine Law)
                // For diffuse emission, the probability distribution is proportional to cos(θ)
                // Using inverse CDF: θ = arcsin(2u - 1) where u ~ Uniform(0,1)
                let u = std_rng.gen_range(0.0..1.0);
                let theta_radians = FloatType::asin(2.0 * u - 1.0); // Range: [-π/2, π/2]
                
                let angle_of_ray = normal_angle_deg + theta_radians.to_degrees();
                Ray::new(new_point, angle_of_ray)
            }
        }
    }
}
