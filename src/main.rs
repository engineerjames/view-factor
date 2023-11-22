use view_factor::*;

// Just to going to start by hard-coding everything
fn main() {
    // Units are in meters
    let line1 = Line {
        points: [Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 1.0 }],
    };

    let line2 = Line {
        points: [Point { x: 0.5, y: 0.0 }, Point { x: 1.5, y: 0.0 }],
    };

    println!("{:?}", line1);
    println!("{:?}", line2);

    println!("{}", line1.length());
}
