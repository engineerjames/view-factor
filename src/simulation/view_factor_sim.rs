type FloatType = f32;

pub fn dot(a: &Point2D, b: &Point2D) -> FloatType {
    (a.x * b.x) + (a.y * b.y)
}

pub fn dist(a: &Point2D, b: &Point2D) -> FloatType {
    FloatType::sqrt(FloatType::powf(b.x - a.x, 2.0) + FloatType::powf(b.y - a.y, 2.0))
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

pub struct Line2DState {
    pub normals: [Point2D; 2],
    pub points: [Point2D; 2], // Could use multiple constructors here eventually
    pub midpoint: Point2D,
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

        Line2DState {
            normals: [normal_1, normal_2],
            points: [point1, point2],
            midpoint: midpoint,
        }
    }
}

pub enum ShapeType {
    Line2D(Line2DState), // Just one shape for now
}

#[derive(Debug)]
pub struct NormalIndexMap {
    source_shape_index: usize,
    source_normal_index: usize,
    target_shape_index: usize,
    target_normal_index: usize,
    distance: FloatType,
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
    // TODO: Logger
}

impl Simulation {
    pub fn new(num_emissions: u64, random_seed: Option<u64>) -> Simulation {
        Simulation {
            emitting_shapes: Vec::new(),
            number_of_emissions: num_emissions,
            random_seed: random_seed.unwrap_or_default(),
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
                                source_normal_index: source_norm_index,
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

    pub fn run(self: &Self) {
        println!("{}", self.emitting_shapes.len());
        println!("{:?}", self.emitting_shapes[0].emits_to);
        println!("{:?}", self.emitting_shapes[1].emits_to);
    }
}

mod tests {

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
        let new_point = Line2DState::new(Point2D { x: 1.0, y: 1.0 }, Point2D { x: 2.0, y: 2.0 });

        assert_eq!(new_point.midpoint, Point2D { x: 1.5, y: 1.5 });

        let emissive_shape =
            EmissiveShape::new(String::from("EmissiveTest1"), ShapeType::Line2D(new_point));

        assert_eq!(emissive_shape.name, String::from("EmissiveTest1"));
    }
}
