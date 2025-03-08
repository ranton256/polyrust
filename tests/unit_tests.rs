
use polyrust::{Point, Segment};
use polyrust::intersect_line_segments;

#[test]
fn test_intersect_line_segments() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let p3 = Point { x: 1.0, y: 2.0 };
    let p4 = Point { x: 2.0, y: 1.0 };
    let line_one = Segment::new(p1, p2);
    let line_two = Segment::new(p3, p4);
    let intersection = intersect_line_segments(&line_one, &line_two);
    assert_eq!(intersection, Some(Point { x: 1.5, y: 1.5 }));
}

#[test]
fn test_parallel_no_intersection() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let p3 = Point { x: 3.0, y: 3.0 };
    let p4 = Point { x: 4.0, y: 4.0 };
    let line_one = Segment::new(p1, p2);
    let line_two = Segment::new(p3, p4);
    let intersection = intersect_line_segments(&line_one, &line_two);
    assert_eq!(intersection, None);
}

#[test]
fn test_point_on_line_not_on_segment() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let p3 = Point { x: 0.0, y: 3.0 };
    let p4 = Point { x: 2.0, y: 3.0 };
    let line_one = Segment::new(p1, p2);
    let line_two = Segment::new(p3, p4);
    let intersection = intersect_line_segments(&line_one, &line_two);
    assert_eq!(intersection, None);
}
