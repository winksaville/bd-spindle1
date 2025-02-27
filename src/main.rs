use std::env;

use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    // Check for the correct number of command line arguments
    if env::args().len() != 5 {
        eprintln!("Usage: bd-spindle dot_diameter dot_spacing spindle_height segments");
        std::process::exit(1);
    }

    // Constants
    const WALL_THICKNESS: f64 = 0.5;
    const DOT_CLEARANCE: f64 = 0.1;
    const FLANGE_HEIGHT: f64 = 2.0;

    // Parse command line arguments to get the spindle dimensions
    let args: Vec<String> = env::args().collect();
    let dot_diameter= args[1].parse::<f64>().unwrap();
    let dot_spacing= args[2].parse::<f64>().unwrap();
    let spindle_height = args[3].parse::<f64>().unwrap();
    let segments= args[4].parse::<usize>().unwrap();

    // Create flanges
    let flange_width = dot_spacing;
    let flange_length = dot_spacing;

    let bottom_flange = CSG::cube(flange_width, flange_length, FLANGE_HEIGHT, None);
    let bottom_flange = bottom_flange.clone().translate(Vector3::new(-flange_width / 2.0, -flange_length / 2.0, 0.0));
    let top_flange = bottom_flange.clone().translate(Vector3::new(0.0, 0.0, spindle_height - FLANGE_HEIGHT));

    // Create cylinder
    let inner_radius = (dot_diameter / 2.0) + DOT_CLEARANCE;
    let outer_radius = inner_radius + WALL_THICKNESS;
    let spindle_shaft = CSG::cylinder(outer_radius, spindle_height, segments, None);  // 1 x 20 cylinder

    // This will be the hole for the dot pin
    let spindle_hole= CSG::cylinder(inner_radius, spindle_height, segments, None);  // 1 x 20 cylinder

    // Union the flanges with the spindle shaft:
    let spindle_with_flanges= spindle_shaft.union(&bottom_flange).union(&top_flange);

    // Remove the material for the solenoid pin
    let spindle = spindle_with_flanges.difference(&spindle_hole);


    // Write the result as an ASCII STL:
    let name = &format!("spindle1.dot_diameter{:0.2}_dot_spacing{:0.2}_spindle_height{:0.2}_segments{:0.2}", dot_diameter, dot_spacing, spindle_height, segments);
    let stl = spindle.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
