extern crate view_factor;

mod integration_tests {
    use view_factor::simulation::view_factor_sim::{
        EmissiveShape, Line2DState, Point2D, ShapeType, Simulation,
    };

    use view_factor::simulation::view_factor_analytic::{
        parallel_strips, unequal_normal_adjacent_strips,
    };

    #[test]
    fn integration_test_parallel_strips() {
        let mut sim: Simulation = Simulation::new(500000, Some(2342));

        // Create four points that represents our two lines
        let p1 = Point2D::new((0.0, -5.0));
        let p2 = Point2D::new((0.0, 5.0));
        let p3 = Point2D::new((2.0, -5.0));
        let p4 = Point2D::new((2.0, 5.0));

        // Add two lines for starters
        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line"),
            ShapeType::Line2D(Line2DState::new(p1, p2)),
        )));

        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line_to_the_right"),
            ShapeType::Line2D(Line2DState::new(p3, p4)),
        )));

        sim.configure();

        let results = sim.run();

        for (i, result) in results.iter().enumerate() {
            println!("i={}, shape_name={}", i, result.emitting_shape.name);
            for (j, view_factor) in result.normal_index_to_view_factor.iter().enumerate() {
                println!("j={} view_factor={}", j, view_factor.1);
            }
        }

        let analytic_result = parallel_strips(2.0, 10.0, 10.0);
        println!("analytic_result={}", analytic_result);
    }

    #[test]
    fn integration_test_adjacent_strips() {
        let mut sim: Simulation = Simulation::new(500000, Some(2342));

        // Create four points that represents our two lines
        let p1 = Point2D::new((0.0, 0.0));
        let p2 = Point2D::new((5.0, 0.0));
        let p3 = Point2D::new((0.0, 0.0));
        let p4 = Point2D::new((0.0, 5.0));

        // Add two lines for starters
        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line"),
            ShapeType::Line2D(Line2DState::new(p1, p2)),
        )));

        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line_to_the_right"),
            ShapeType::Line2D(Line2DState::new(p3, p4)),
        )));

        sim.configure();

        let results = sim.run();

        for (i, result) in results.iter().enumerate() {
            println!("i={}, shape_name={}", i, result.emitting_shape.name);
            for (j, view_factor) in result.normal_index_to_view_factor.iter().enumerate() {
                println!("j={} view_factor={}", j, view_factor.1);
            }
        }

        let analytic_result = unequal_normal_adjacent_strips(5.0, 5.0);
        println!("analytic_result={}", analytic_result);
    }

    #[test]
    fn same_length_lines_have_equal_view_factors() {
        let mut sim: Simulation = Simulation::new(500000, Some(2342));

        // Create four points that represents our two lines
        let p1 = Point2D::new((0.0, -5.0));
        let p2 = Point2D::new((0.0, 5.0));
        let p3 = Point2D::new((2.0, -5.0));
        let p4 = Point2D::new((2.0, 5.0));

        // Add two lines for starters
        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line"),
            ShapeType::Line2D(Line2DState::new(p1, p2)),
        )));

        sim.add_shape(Box::new(EmissiveShape::new(
            String::from("straight_line_to_the_right"),
            ShapeType::Line2D(Line2DState::new(p3, p4)),
        )));

        sim.configure();

        let results = sim.run();

        for (i, result) in results.iter().enumerate() {
            println!("i={}, shape_name={}", i, result.emitting_shape.name);
            for (j, view_factor) in result.normal_index_to_view_factor.iter().enumerate() {
                println!("j={} view_factor={}", j, view_factor.1);
            }
        }

        // Assert that the view factors are equal to eachother.
    }
    
}
