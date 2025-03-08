
use polyrust::{Point, Segment};
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
}