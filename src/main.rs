mod simulation;

use simulation::view_factor_sim::{EmissiveShape, Line2DState, Point2D, ShapeType, Simulation};

// Just to going to start by hard-coding everything
fn main() {
    // Eventually take in arguments from the command line or JSON file?
    let mut sim: Simulation = Simulation::new(5000000, Some(2342));

    // Create four points that represents our two lines
    let p1 = Point2D::new((1.0, 2.0));
    let p2 = Point2D::new((3.0, 4.0));
    let p3 = Point2D::new((-20.0, 4.0));
    let p4 = Point2D::new((-20.0, 7.0));

    // Add two lines for starters
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
