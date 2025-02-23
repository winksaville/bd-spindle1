use std::env;

fn main() {
    // Alias the library’s generic CSG type with empty metadata:
    type CSG = csgrs::csg::CSG<()>;

    let args: Vec<String> = env::args().collect();
    let inner_radius = args[1].parse::<f64>().unwrap();
    let outer_radius = args[2].parse::<f64>().unwrap();
    let height = args[3].parse::<f64>().unwrap();
    let segments= args[4].parse::<usize>().unwrap();

    // Create two shapes:
    let inner_cylinder = CSG::cylinder(inner_radius, height, segments, None);  // 1 x 20 cylinder
    let outer_cylinder = CSG::cylinder(outer_radius, height, segments, None);  // 1 x 20 cylinder

    // Difference one from the other:
    let difference_result = outer_cylinder.difference(&inner_cylinder);

    // Write the result as an ASCII STL:
    let name = &format!("cylinder_inner{:0.2}_outer{:0.2}_height{:0.2}_segments{:0.2}", inner_radius, outer_radius, height, segments);
    let stl = difference_result.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
