use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

type FloatType = f32;

const EPSILON: FloatType = FloatType::EPSILON * 4.0;

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

pub enum ShapeType {
    Line2D(Line2DState), // Just one shape for now
}

pub struct Ray {
    pub point: Point2D,
    pub angle: FloatType,
}

impl Ray {
    fn new(p: Point2D, angle: FloatType) -> Ray {
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

    fn set_inward_normal(&mut self, center: &Point2D) {
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

    pub fn get_emissive_ray(&self, std_rng: &mut StdRng) -> Ray {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => {
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

                // The ray we fire should be the same angle as the normal +/- 90 degrees
                let angle_deg =
                    FloatType::atan2(source_shape_normal.y, source_shape_normal.x).to_degrees();
                let min_angle_deg = angle_deg - 90.0;
                let max_angle_deg = angle_deg + 90.0;

                let angle_of_ray = std_rng.gen_range(min_angle_deg..max_angle_deg);
                Ray::new(new_point, angle_of_ray)
            }
        }
    }

    pub fn get_normals(&self) -> &[Point2D; 2] {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => &line_state.normals,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewFactorResult {
    pub from_shape: String,
    pub to_shape: String,
    pub view_factor: FloatType,
}

pub struct Simulation {
    pub emitting_shapes: Vec<Box<EmissiveShape>>,
    pub number_of_emissions: u64,
    pub random_seed: u64,
    rng: StdRng,
    // TODO: Logger
}

impl Simulation {
    pub fn new(num_emissions: u64, random_seed: Option<u64>) -> Simulation {
        Simulation {
            emitting_shapes: Vec::new(),
            number_of_emissions: num_emissions,
            random_seed: random_seed.unwrap_or_default(),
            rng: StdRng::seed_from_u64(random_seed.unwrap_or_default()),
        }
    }

    pub fn add_shape(&mut self, shape: Box<EmissiveShape>) {
        self.emitting_shapes.push(shape);
    }

    pub fn configure(&mut self) {
        // Calculate the center of all shapes
        let center = self.calculate_center();
        
        // Set each shape's inward-facing normal (toward the center)
        for shape in &mut self.emitting_shapes {
            shape.set_inward_normal(&center);
        }
    }

    fn calculate_center(&self) -> Point2D {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for shape in &self.emitting_shapes {
            match &shape.shape_type {
                ShapeType::Line2D(line_state) => {
                    sum_x += line_state.midpoint.x;
                    sum_y += line_state.midpoint.y;
                    count += 1;
                }
            }
        }

        if count > 0 {
            Point2D {
                x: sum_x / (count as FloatType),
                y: sum_y / (count as FloatType),
            }
        } else {
            Point2D { x: 0.0, y: 0.0 }
        }
    }

    pub fn run(&mut self) -> Vec<ViewFactorResult> {
        let mut results = Vec::new();
        
        for emitting_shape in &self.emitting_shapes {
            println!("Processing emissions for shape {}", emitting_shape.name);

            // Track hits per target shape for this emitting shape
            let mut hit_counts: HashMap<String, u64> = HashMap::new();

            // Emit rays only from the outward-facing normal
            for _ in 0..self.number_of_emissions {
                let ray = emitting_shape.get_emissive_ray(&mut self.rng);
                
                if let Some(hit_shape) = does_ray_hit(&ray, &self.emitting_shapes, emitting_shape) {
                    *hit_counts.entry(hit_shape.name.clone()).or_insert(0) += 1;
                }
            }

            // Calculate view factors from this shape to each target shape
            for (target_name, hit_count) in hit_counts.iter() {
                let view_factor = (*hit_count as FloatType) / (self.number_of_emissions as FloatType);
                
                println!(
                    "View Factor F_{{{} -> {}}} = {:.6}",
                    emitting_shape.name, target_name, view_factor
                );
                
                results.push(ViewFactorResult {
                    from_shape: emitting_shape.name.clone(),
                    to_shape: target_name.clone(),
                    view_factor,
                });
            }
        }
        
        results
    }
}

fn does_ray_hit<'a>(
    ray: &Ray,
    emitting_shapes: &'a [Box<EmissiveShape>],
    emitted_from: &'a Box<EmissiveShape>,
) -> Option<&'a Box<EmissiveShape>> {
    for shape in emitting_shapes {
        // Don't check to see if we intersect with ourselves--we always will
        if shape.name == emitted_from.name {
            continue;
        }

        match &shape.shape_type {
            ShapeType::Line2D(line_state) => {
                // https://gamedev.stackexchange.com/questions/109420/ray-segment-intersection
                // dx = change in x for ray
                // dy = change in y for ray
                // x,y = origin of ray
                // x1,y1, x2,y2 = line segment
                // TODO: Make sure the lines aren't parallel
                let dx = FloatType::cos(ray.angle.to_radians());
                let dy = FloatType::sin(ray.angle.to_radians());

                let x2_minus_x1 = line_state.points[1].x - line_state.points[0].x;
                let y2_minus_y1 = line_state.points[1].y - line_state.points[0].y;

                // Check if lines are not parallel using epsilon comparison
                let cross_product = dy * x2_minus_x1 - dx * y2_minus_y1;
                if FloatType::abs(cross_product) > EPSILON {
                    let d = (dx * y2_minus_y1) - (dy * x2_minus_x1);
                    let y_minus_y1: f32 = ray.point.y - line_state.points[0].y;
                    let x_minus_x1 = ray.point.x - line_state.points[0].x;

                    if FloatType::abs(d) > EPSILON {
                        let r = (((y_minus_y1) * (x2_minus_x1)) - (x_minus_x1) * (y2_minus_y1)) / d;
                        let s = (((y_minus_y1) * dx) - (x_minus_x1 * dy)) / d;

                        if r >= 0.0 && s >= 0.0 && s <= 1.0 {
                            return Some(shape);
                        }
                    } else {
                        println!("D was actually equal to 0! ERROR")
                    }
                }

                continue;
            }
        }
    }

    None
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn line_state_check() {
        let new_point = Line2DState::new(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 2.0, y: 2.0 });

        assert_eq!(new_point.points[0].x, 1.0);
        assert_eq!(new_point.points[0].y, 1.0);
        assert_eq!(new_point.points[1].x, 2.0);
        assert_eq!(new_point.points[1].y, 2.0);
    }

    #[test]
    fn line_state_check_midpoint() {
        let new_line = Line2DState::new(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 2.0, y: 2.0 });

        assert_eq!(new_line.midpoint, Point2D { x: 1.5, y: 1.5 });

        let emissive_shape =
            EmissiveShape::new(String::from("EmissiveTest1"), ShapeType::Line2D(new_line));

        assert_eq!(emissive_shape.name, String::from("EmissiveTest1"));
    }

    #[test]
    fn straight_line_has_zero_slope() {
        let new_line = Line2DState::new(Point2D::new((-1.0, 2.0)), Point2D::new((-1.0, 4.0)));

        assert_eq!(new_line.slope, 0.0);
        assert_eq!(FloatType::atan(new_line.slope).to_degrees(), 0.0);
    }
}
