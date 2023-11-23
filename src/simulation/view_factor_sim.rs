use num::Float;
use crate::Point2D;

pub trait EmissiveShape<T: Float> {
    fn get_normal(&self) -> [Point2D<T>; 2];
    fn set_normal(&mut self, i: u32);

    fn set_name(&mut self, name: String);
    fn get_name(&self) -> String;
}

pub struct SimulationParameters<T: Float> {
    pub emitting_shapes: Vec<Box<dyn EmissiveShape<T>>>,
    pub number_of_emissions: u64,
    pub random_seed: u64,
}
