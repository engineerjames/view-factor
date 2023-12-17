use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

use num::Float;

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

pub fn dot<T: Float>(a: &Point2D<T>, b: &Point2D<T>) -> T {
    (a.x * b.x) + (a.y * b.y)
}

#[derive(Debug)]
pub struct Point2D<T: num::Float> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Float,
{
    pub fn new((x1, y1): (T, T)) -> Point2D<T> {
        Point2D { x: x1, y: y1 }
    }
}

pub struct Line2DState<T: Float> {
    pub normals: [Point2D<T>; 2],
    pub points: [Point2D<T>; 2], // Could use multiple constructors here eventually
}

impl<T> Line2DState<T>
where
    T: Float,
{
    pub fn new(point1: Point2D<T>, point2: Point2D<T>) -> Line2DState<T> {
        let dy = point2.y - point1.y;
        let dx = point2.x - point1.x;

        let normal_1 = Point2D::new((-dy, dx));
        let normal_2 = Point2D::new((dy, -dx));

        Line2DState {
            normals: [normal_1, normal_2],
            points: [point1, point2],
        }
    }
}

pub enum ShapeType<T: Float> {
    Line2D(Line2DState<T>), // Just one shape for now
}

// TODO: You just wrote this, need to plugin to check shape with others
pub fn get_normals_from_shape<T: Float>(shape: &ShapeType<T>) -> &[Point2D<T>; 2] {
    match shape {
        ShapeType::Line2D(line_state) => &line_state.normals,
    }
}

pub struct EmissiveShape<T: Float> {
    pub name: String,
    pub shape_type: ShapeType<T>,
    pub id: u64,
    // Hash map between the normal index of the given shape, which maps
    // to a list of pair u64's.  Each pair signifies:
    // (target_shape_id, normal_index)
    pub emits_to: HashMap<u64, Vec<(u64, u64)>>,
}

impl<T> EmissiveShape<T>
where
    T: Float,
{
    // Generic constructor?
    pub fn new(name: String, shape_type: ShapeType<T>) -> EmissiveShape<T> {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            id: unique_id(),
            emits_to: HashMap::new(),
        }
    }
}

pub struct Simulation<T: Float> {
    pub emitting_shapes: Vec<Box<EmissiveShape<T>>>,
    pub number_of_emissions: u64,
    pub random_seed: u64,
    // TODO: Logger
}

impl<T> Simulation<T>
where
    T: Float,
{
    pub fn new(num_emissions: u64, random_seed: Option<u64>) -> Simulation<T> {
        Simulation {
            emitting_shapes: Vec::new(),
            number_of_emissions: num_emissions,
            random_seed: random_seed.unwrap_or_default(),
        }
    }

    pub fn add_shape(self: &mut Self, shape: Box<EmissiveShape<T>>) {
        self.emitting_shapes.push(shape);
    }

    // TODO: Come up with better name
    fn check_shape_against_others(self: &Self, shape_to_check: &Box<EmissiveShape<T>>) {
        for shape in &self.emitting_shapes {
            if shape.id == shape_to_check.id {
                continue;
            }

            let norms_to_check = get_normals_from_shape(&shape_to_check.shape_type);
            let norms = get_normals_from_shape(&shape.shape_type);

            for n_to_check in norms_to_check {
                for n in norms {
                    if Float::is_sign_positive(dot(n_to_check, n)) {
                        // HIT TODO FILL THIS OUT
                        println!("hit");
                    }
                }
            }
        }
    }

    pub fn configure(self: &Self) {
        for shape in &self.emitting_shapes {
            println!("{}", shape.name);
            self.check_shape_against_others(shape);
        }
    }

    pub fn run(self: &Self) {
        println!("{}", self.emitting_shapes.len());
    }
}
