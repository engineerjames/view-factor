use num::Float;

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
    normals: [Point2D<T>; 2],
    points: [Point2D<T>; 2], // Could use multiple constructors here eventually
}

enum ShapeType<T: Float> {
    Line2D(Line2DState<T>), // Just one shape for now
}

pub struct EmissiveShape<T: Float> {
    name: String,
    shape_type: ShapeType<T>,
    id: u64,
}

impl<T> EmissiveShape<T>
where
    T: Float,
{
    // Generic constructor?
    fn new(name: String, shape_type: ShapeType<T>, id: u64) -> EmissiveShape<T> {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            id: id,
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

    // Instead of set_normals()
    pub fn configure(self: &Self) {
        println!("{}", self.emitting_shapes.len());
    }

    pub fn run(self: &Self) {
        println!("{}", self.emitting_shapes.len());
    }
}
