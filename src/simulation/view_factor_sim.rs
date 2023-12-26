use rand::{rngs::StdRng, Rng, SeedableRng};

type FloatType = f32;

pub fn dot(a: &Point2D, b: &Point2D) -> FloatType {
    (a.x * b.x) + (a.y * b.y)
}

pub fn cross(a: &Point2D, b: &Point2D) -> FloatType {
    (a.x * b.y) - (a.y * b.x)
}

pub fn dist(a: &Point2D, b: &Point2D) -> FloatType {
    FloatType::sqrt(FloatType::powf(b.x - a.x, 2.0) + FloatType::powf(b.y - a.y, 2.0))
}

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
            normals: [normal_1, normal_2],
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

#[derive(Debug)]
pub struct NormalIndexMap {
    source_shape_index: usize,
    target_shape_index: usize,
    target_normal_index: usize,
    distance: FloatType,
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
    // Hash map between the normal index of the given shape, which maps
    // to a list of pair u64's.  Each pair signifies:
    // (target_shape_id, normal_index)
    pub emits_to: Vec<NormalIndexMap>,
}

impl EmissiveShape {
    // Generic constructor?
    pub fn new(name: String, shape_type: ShapeType) -> EmissiveShape {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            emits_to: Vec::new(),
        }
    }

    pub fn get_emissive_ray(
        self: &Self,
        std_rng: &mut StdRng,
        target_normal_map: &NormalIndexMap,
        emissive_shapes: &[Box<EmissiveShape>],
    ) -> Ray {
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

                let target_shape = &emissive_shapes[target_normal_map.target_shape_index];
                let target_shape_normal =
                    &target_shape.get_normals()[target_normal_map.target_normal_index];

                let normal_as_line =
                    Line2DState::new(Point2D { x: 0.0, y: 0.0 }, target_shape_normal.clone());

                let min_angle_deg = FloatType::atan(normal_as_line.slope).to_degrees();
                let max_angle_deg = min_angle_deg + 180.0;

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

    pub fn get_reference_translated_normals(self: &Self) -> [Point2D; 2] {
        match &self.shape_type {
            ShapeType::Line2D(line_state) => [
                &line_state.normals[0] + &line_state.midpoint,
                &line_state.normals[1] + &line_state.midpoint,
            ],
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
        for i in 0..self.emitting_shapes.len() {
            let mut new_mapping: Vec<NormalIndexMap> = Vec::new();
            let source_shape = self.emitting_shapes[i].as_ref();
            let source_normals = source_shape.get_normals();

            for j in 0..self.emitting_shapes.len() {
                // Don't check a shape against itself
                if i == j {
                    continue;
                }

                let target_shape = self.emitting_shapes[j].as_ref();
                let target_normals = target_shape.get_normals();

                for (source_norm_index, source_normal) in source_normals.iter().enumerate() {
                    for (target_norm_index, target_normal) in target_normals.iter().enumerate() {
                        if dot(source_normal, target_normal) < 0.0 {
                            let distance = dist(
                                &source_shape.get_reference_translated_normals()[source_norm_index],
                                &target_shape.get_reference_translated_normals()[target_norm_index],
                            );

                            let new_source_target_pair = NormalIndexMap {
                                source_shape_index: i,
                                target_shape_index: j,
                                target_normal_index: target_norm_index,
                                distance: distance,
                            };

                            let prev_source_target_pair_index = new_mapping.iter().position(|x| {
                                x.source_shape_index == i && x.target_shape_index == j
                            });

                            if prev_source_target_pair_index.is_some() {
                                let prev_source_target_pair =
                                    &new_mapping[prev_source_target_pair_index.unwrap()];
                                let should_replace_value =
                                    distance < prev_source_target_pair.distance;

                                if should_replace_value {
                                    new_mapping[prev_source_target_pair_index.unwrap()] =
                                        new_source_target_pair;
                                }
                            } else {
                                new_mapping.push(new_source_target_pair);
                            }
                        }
                    }
                }
            }

            self.emitting_shapes[i].as_mut().emits_to = new_mapping;
        }
    }

    pub fn run(self: &mut Self) {
        println!("{}", self.emitting_shapes.len());
        println!("{:?}", self.emitting_shapes[0].emits_to);
        println!("{:?}", self.emitting_shapes[1].emits_to);

        // Update back to number_of_emissions

        // Will need to iterate across shapes that we're emitting to?
        for emitting_shape in &self.emitting_shapes {
            for target_normal_map in &emitting_shape.emits_to {
                let mut hit_count: u64 = 0;
                for _ in 0..self.number_of_emissions {
                    let s = emitting_shape.get_emissive_ray(
                        &mut self.rng,
                        target_normal_map,
                        &self.emitting_shapes,
                    );
                    //println!("x={} y={}, theta={}", s.point.x, s.point.y, s.angle);
                    let does_hit = does_ray_hit(&s, &self.emitting_shapes);

                    if does_hit.is_some() {
                        hit_count += 1;
                    }
                }

                println!(
                    "Hit Ratio = {}",
                    (hit_count as FloatType) / (self.number_of_emissions as FloatType)
                );
            }
        }
    }
}

fn does_ray_hit<'a>(
    ray: &Ray,
    emitting_shapes: &'a [Box<EmissiveShape>],
) -> Option<&'a Box<EmissiveShape>> {
    for shape in emitting_shapes {
        match &shape.shape_type {
            ShapeType::Line2D(line_state) => {
                // q = x1/y1 (point 1 of the line)
                // q + s = x2/y2 (point 2 of the line)
                // Then your line segment intersects the ray if 0 ≤ t and 0 ≤ u ≤ 1.
                let r = Point2D::new((FloatType::cos(ray.angle), FloatType::sin(ray.angle)));
                let q = &line_state.points[0];
                let p = &ray.point;
                let s = Point2D::new((
                    line_state.points[1].x - line_state.points[0].x,
                    line_state.points[1].y - line_state.points[0].y,
                ));

                //let t = (q - p) x s / (r x s)
                //let u = (q − p) × r / (r × s)
                let r_cross_s = cross(&r, &s);

                if FloatType::abs(r_cross_s) <= (FloatType::EPSILON * 4.0) {
                    return None;
                }

                let t = cross(&(q - p), &s) / r_cross_s;
                let u = cross(&(q - p), &r) / r_cross_s;

                if (0.0 <= t && t <= 1.0) && (0.0 <= u && u <= 1.0) {
                    return Some(shape);
                } else {
                    ()
                }
            }
        }
    }

    None
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn check_dot_product_calculation() {
        let a = Point2D { x: 1.0, y: 2.0 };
        let b = Point2D { x: 1.0, y: 2.0 };
        let actual_result = dot(&a, &b);
        let expected_result = 5.0;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn check_distance_calculation() {
        let a = Point2D { x: 1.0, y: 1.0 };
        let b = Point2D { x: 2.0, y: 2.0 };

        let actual_result = dist(&a, &b);
        let expected_result = f32::sqrt(2.0);

        assert_eq!(actual_result, expected_result);
    }

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
