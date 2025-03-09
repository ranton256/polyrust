
use polyrust::{generate_svg_from_polygons, intersect_convex_polygons, ConvexPolygon, Point, Segment};
use polyrust::intersect_line_segments;

fn main() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let p3 = Point { x: 1.0, y: 2.0 };
    let p4 = Point { x: 2.0, y: 1.0 };

    let line_one = Segment::new(p1, p2);
    let line_two = Segment::new(p3, p4);

    let intersection = intersect_line_segments(&line_one, &line_two);
    match intersection {
        Some(point) => println!("Intersection at {:?}", point),
        None => println!("No intersection"),
    }

    // create two polygons
    let poly_one = ConvexPolygon::new(&vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 0.0, y: 2.0 },
    ]);
    let poly_two = ConvexPolygon::new(&vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 3.0, y: 1.0 },
        Point { x: 3.0, y: 3.0 },
        // Point { x: 1.0, y: 3.0 },
    ]);
    // calculate the intersection of the two polygons
    let intersection_points = intersect_convex_polygons(&poly_one, &poly_two);
    let poly_three = ConvexPolygon::new(&intersection_points);
    // print the intersection points
    println!("Intersection points: {:?}", intersection_points);
    
    let pad = 0.1;
    let min = Point { x: 0.0 - pad, y: 0.0 - pad};
    let max = Point { x: 3.0 + pad, y: 3.0 + pad};

    // generate an SVG file of the polygons and their intersection
    let svg_content = generate_svg_from_polygons(&vec![&poly_one, &poly_two, &poly_three],
        &vec![ "red", "blue", "green" ],
        100, 100,
        Some((min, max))
        );

    // write the SVG content to a file
    std::fs::write("poly_output.svg", svg_content).expect("Failed to write SVG file");
    println!("SVG file generated successfully.");

    

}