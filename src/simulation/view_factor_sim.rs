use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

type FloatType = f32;

// TODO: Do we need this? This will always match the order in the Vec...
pub fn unique_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let id = COUNTER.fetch_add(1, SeqCst);
    assert_ne!(
        id,
        u64::MAX,
        "ID counter has overflowed and is no longer unique"
    );
    id
}

pub fn dot(a: &Point2D, b: &Point2D) -> FloatType {
    (a.x * b.x) + (a.y * b.y)
}

pub fn dist(a: &Point2D, b: &Point2D) -> FloatType {
    FloatType::sqrt(FloatType::powf(b.x - a.x, 2.0) + FloatType::powf(b.y - a.y, 2.0))
}

#[derive(Debug, PartialEq)]
pub struct Point2D {
    pub x: FloatType,
    pub y: FloatType,
}

impl Point2D {
    pub fn new((x1, y1): (FloatType, FloatType)) -> Point2D {
        Point2D { x: x1, y: y1 }
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

// TODO: You just wrote this, need to plugin to check shape with others
pub fn get_normals_from_shape(shape: &ShapeType) -> &[Point2D; 2] {
    match shape {
        ShapeType::Line2D(line_state) => &line_state.normals,
    }
}

pub type ShapeIdToNormalIndexPair = (u64, usize);

pub struct EmissiveShape {
    pub name: String,
    pub shape_type: ShapeType,
    pub id: u64,
    // Hash map between the normal index of the given shape, which maps
    // to a list of pair u64's.  Each pair signifies:
    // (target_shape_id, normal_index)
    pub emits_to: HashMap<usize, Vec<ShapeIdToNormalIndexPair>>,
}

impl EmissiveShape {
    // Generic constructor?
    pub fn new(name: String, shape_type: ShapeType) -> EmissiveShape {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            id: unique_id(),
            emits_to: HashMap::new(),
        }
    }

    pub fn get_reference_point(self: Self) -> Point2D {
        match self.shape_type {
            ShapeType::Line2D(line_state) => line_state.midpoint,
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
            let mut new_mapping: HashMap<usize, Vec<ShapeIdToNormalIndexPair>> = HashMap::new();

            for j in 0..self.emitting_shapes.len() {
                let shape_to_check = self.emitting_shapes[i].as_ref();
                let shape = self.emitting_shapes[j].as_ref();
                if shape_to_check.id == shape.id {
                    continue;
                }

                let norms_to_check = get_normals_from_shape(&shape_to_check.shape_type);
                let norms = get_normals_from_shape(&shape.shape_type);

                for (n_to_check_index, n_to_check) in norms_to_check.iter().enumerate() {
                    for (n_index, n) in norms.iter().enumerate() {
                        if dot(n_to_check, n) < 0.0 {
                            if !new_mapping.contains_key(&n_to_check_index) {
                                new_mapping.insert(n_to_check_index, Vec::new());
                            }

                            new_mapping
                                .get_mut(&n_to_check_index)
                                .unwrap()
                                .push((shape.id.clone(), n_index.clone()));
                        }
                    }
                }
            }

            self.emitting_shapes[i].as_mut().emits_to = new_mapping;
        }
    }

    pub fn run(self: &Self) {
        println!("{}", self.emitting_shapes.len());
        println!("{:?}", self.emitting_shapes[0].emits_to.keys());
        println!("{:?}", self.emitting_shapes[1].emits_to.keys());
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
    }
}
