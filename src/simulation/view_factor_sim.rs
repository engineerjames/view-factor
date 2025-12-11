use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;

use super::geometry::{Point2D, FloatType, EPSILON, Ray};
pub use super::geometry::{ShapeType, EmissiveShape};

#[derive(Debug, Clone)]
pub struct ViewFactorResult {
    pub from_shape: String,
    pub to_shape: String,
    pub view_factor: FloatType,
}

pub struct Simulation {
    pub emitting_shapes: Vec<Box<EmissiveShape>>,
    pub number_of_emissions: u64,
    rng: StdRng,
    // TODO: Logger
}

impl Simulation {
    pub fn new(num_emissions: u64, random_seed: Option<u64>) -> Simulation {
        Simulation {
            emitting_shapes: Vec::new(),
            number_of_emissions: num_emissions,
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
    use crate::simulation::geometry::{Point2D, Line2DState, FloatType};
    use super::{EmissiveShape, ShapeType};

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
