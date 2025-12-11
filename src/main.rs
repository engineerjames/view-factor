mod simulation;
use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use simulation::geometry::{Line2DState, Point2D, EmissiveShape, ShapeType};
use simulation::view_factor_sim::Simulation;
use log4rs::{self, Config};

fn setup_logging(file_path: &str) -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    Ok(())
}

// Just to going to start by hard-coding everything
fn main() {
    // Eventually take in arguments from the command line or JSON file?
    let mut sim: Simulation = Simulation::new(50000, Some(2342));

    // Set up logging
    setup_logging("view_factor_sim.log").unwrap_or_else(|e| {
        println!("Failed to setup logging: {}", e);
    });

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
