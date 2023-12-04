use crate::Point2D;
use num::Float;

pub struct Line2DState {
    normals: i32,
}

enum ShapeType {
    Line2D(Line2DState),
}

pub struct EmissiveShape<T: Float> {
    name: String,
    shape_type: ShapeType,
    id: u64,
}

impl<T> EmissiveShape<T>
where
    T: Float,
{
    fn new(name: String, shape_type: ShapeType, id: u64) -> EmissiveShape<T> {
        EmissiveShape {
            name: name,
            shape_type: shape_type,
            id: id,
        }
    }
}

pub struct SimulationParameters<T: Float> {
    pub emitting_shapes: Vec<Box<EmissiveShape<T>>>,
    pub number_of_emissions: u64,
    pub random_seed: u64,
    // TODO: Logger
}

impl<T> SimulationParameters<T>
where
    T: Float,
{
    fn new(num_emissions: u64, random_seed: u64) -> SimulationParameters<T> {
        SimulationParameters {
            emitting_shapes: Vec::new(),
            number_of_emissions: num_emissions,
            random_seed: random_seed,
        }
    }
    // Instead of set_normals()
    fn configure() {
        todo!("Implement");
    }

    fn run() {
        todo!("Implement");
    }
}
