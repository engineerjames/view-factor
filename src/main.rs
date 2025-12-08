mod simulation;

use simulation::view_factor_sim::{EmissiveShape, Line2DState, Point2D, ShapeType, Simulation};

// Just to going to start by hard-coding everything
fn main() {
    // Eventually take in arguments from the command line or JSON file?
    let mut sim: Simulation = Simulation::new(500000, Some(2342));

    // Create four points that represents our two lines
    // We'll set them up to be unequal normal adjacent strips
    // for which we know the analytic solution.
    let p1 = Point2D::new((0.0, 0.0));
    let p2 = Point2D::new((10.0, 0.0));
    let p3 = Point2D::new((0.0, 0.0));
    let p4 = Point2D::new((0.0, 10.0));

    let analytic_solution =
        simulation::view_factor_analytic::unequal_normal_adjacent_strips(10.0, 10.0);

    println!("Analytic solution for unequal normal adjacent strips (10x10): {:.6}", analytic_solution);

    // Add two lines for starters
    // Note: The inward-facing normal (toward the center) will be automatically calculated
    sim.add_shape(Box::new(EmissiveShape::new(
        String::from("angled_line"),
        ShapeType::Line2D(Line2DState::new(p1, p2)),
    )));

    sim.add_shape(Box::new(EmissiveShape::new(
        String::from("straight_line"),
        ShapeType::Line2D(Line2DState::new(p3, p4)),
    )));

    sim.configure();

    let results = sim.run();
    
    println!("\n=== View Factor Results ===");
    for result in results {
        println!(
            "F_{{{} -> {}}} = {:.6}",
            result.from_shape, result.to_shape, result.view_factor
        );
    }
}
