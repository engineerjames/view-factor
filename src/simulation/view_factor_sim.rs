use std::{collections::HashMap, ptr::null};

use rand::{rngs::StdRng, Rng, SeedableRng};

type FloatType = f32;

pub fn is_point_on_line(p: &Point2D, line: &Line2DState) -> bool {
    let result;
    if line.slope != 0.0 {
        result = p.y - (line.slope * p.x + line.y_intercept);
    } else {
        // If we have a straight vertical or horizontal line, we just need
        // to ensure that the new point either has the same X value as BOTH points
        // that make the line, or the same Y value as BOTH points that make the line.
        return (p.x == line.points[0].x && p.x == line.points[1].x)
            || (p.y == line.points[0].y && p.y == line.points[1].y);
    }

    FloatType::abs(result) <= (FloatType::EPSILON * 4.0)
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

    pub fn magnitude(self: &Self) -> FloatType {
        FloatType::sqrt((self.x * self.x) + (self.y * self.y))
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        return Point2D::new((self.x + rhs.x, self.y + rhs.y));
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
        return Point2D::new((self.x - rhs.x, self.y - rhs.y));
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
        if FloatType::abs(dx) >= (FloatType::EPSILON * 4.0) {
            slope = dy / dx;
        }

        let mut y_intercept = 0.0;

        if slope != 0.0 {
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
}

impl EmissiveShape {
    // Generic constructor?
    pub fn new(name: String, shape_type: ShapeType) -> EmissiveShape {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
        }
    }

    pub fn get_emissive_ray(self: &Self, std_rng: &mut StdRng, target_normal_map: &Point2D) -> Ray {
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

                let source_shape_normal = target_normal_map;

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

    pub fn get_normals(self: &Self) -> &[Point2D; 2] {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => &line_state.normals,
        }
    }
}

pub struct Simulation {
    pub emitting_shapes: Vec<Box<EmissiveShape>>,
    pub number_of_emissions: u64,
    pub random_seed: u64,
    rng: StdRng,
    // TODO: Logger
}

pub struct SimulationResult<'a> {
    pub emitting_shape: &'a Box<EmissiveShape>,
    pub normal_index_to_view_factor: HashMap<usize, FloatType>,
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

    pub fn add_shape(self: &mut Self, shape: Box<EmissiveShape>) {
        self.emitting_shapes.push(shape);
    }

    pub fn configure(self: &mut Self) {
        // TODO: Build up a list of all possible shapes that a given emitter can emit to
    }

    pub fn run(self: &mut Self) -> Vec<SimulationResult> {
        let mut sim_results: Vec<SimulationResult> = Vec::new();

        for emitting_shape in &self.emitting_shapes {
            println!("Processing emissions for shape {}", emitting_shape.name);

            let mut normal_to_view_factor: HashMap<usize, FloatType> = HashMap::new();

            for (i, normal) in emitting_shape.get_normals().iter().enumerate() {
                let mut hit_count: u64 = 0;

                for _ in 0..self.number_of_emissions {
                    let s = emitting_shape.get_emissive_ray(&mut self.rng, normal);
                    let does_hit = does_ray_hit(&s, &self.emitting_shapes, emitting_shape);

                    if does_hit.is_some() {
                        hit_count += 1;
                    }
                }

                let view_factor =
                    (hit_count as FloatType) / (self.number_of_emissions as FloatType);

                if view_factor > 0.0 {
                    normal_to_view_factor.insert(i, view_factor);
                }

                println!("Hit Ratio = {}", view_factor);
            }

            sim_results.push(SimulationResult {
                emitting_shape: &emitting_shape,
                normal_index_to_view_factor: normal_to_view_factor,
            })
        }

        sim_results
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

                if dy * x2_minus_x1 != dx * y2_minus_y1 {
                    let d = (dx * y2_minus_y1) - (dy * x2_minus_x1);
                    let y_minus_y1: f32 = ray.point.y - line_state.points[0].y;
                    let x_minus_x1 = ray.point.x - line_state.points[0].x;

                    if FloatType::abs(d) != 0.0 {
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
