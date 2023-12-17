mod simulation;

use simulation::view_factor_sim::Simulation;

// Just to going to start by hard-coding everything
fn main() {
    // Eventually take in arguments from the command line or JSON file?
    let sim: Simulation<f64> = Simulation::new(5000, None);

    sim.configure();

    sim.run();
}
