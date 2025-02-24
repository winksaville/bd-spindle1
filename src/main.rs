use std::env;

use nalgebra::Vector3;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    let args: Vec<String> = env::args().collect();
    let inner_radius = args[1].parse::<f64>().unwrap();
    let outer_radius = args[2].parse::<f64>().unwrap();
    let height = args[3].parse::<f64>().unwrap();
    let segments= args[4].parse::<usize>().unwrap();

    // Create flanges
    let flange_width = 2.5f64;
    let flange_length = flange_width;
    let flange_height = 2.0f64;

    let bottom_flange = CSG::cube(flange_width, flange_length, flange_height, None);
    let bottom_flange = bottom_flange.clone().translate(Vector3::new(-flange_width / 2.0, -flange_length / 2.0, 0.0));
    let top_flange = bottom_flange.clone().translate(Vector3::new(0.0, 0.0, height - flange_height));

    // Create cylinder
    let spindle_shaft = CSG::cylinder(outer_radius, height, segments, None);  // 1 x 20 cylinder

    // This will be the hole for the dot pin
    let spindle_hole= CSG::cylinder(inner_radius, height, segments, None);  // 1 x 20 cylinder

    // Union the flanges with the spindle shaft:
    let spindle_with_flanges= spindle_shaft.union(&bottom_flange).union(&top_flange);

    // Remove the material for the solenoid pin
    let spindle = spindle_with_flanges.difference(&spindle_hole);


    // Write the result as an ASCII STL:
    let name = &format!("spindle.inner{:0.2}_outer{:0.2}_height{:0.2}_segments{:0.2}", inner_radius, outer_radius, height, segments);
    let stl = spindle.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
