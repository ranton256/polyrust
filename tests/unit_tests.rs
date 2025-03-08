
use polyrust::{check_polygon_is_convex, ConvexPolygon, Point, Segment};
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



#[test]
fn test_point_inside_triangle() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let point = Point { x: 1.0, y: 1.0 };
    assert!(polygon.is_point_inside(point));
}

#[test]
fn test_point_outside_triangle() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let point = Point { x: 3.0, y: 3.0 };
    assert!(!polygon.is_point_inside(point));
}

#[test]
fn test_point_on_vertex() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let point = Point { x: 0.0, y: 0.0 };
    assert!(polygon.is_point_inside(point));
}

#[test]
fn test_point_on_edge() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let point = Point { x: 1.0, y: 0.0 };
    assert!(polygon.is_point_inside(point));
}

#[test]
fn test_point_inside_square() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 0.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let point = Point { x: 1.0, y: 1.0 };
    assert!(polygon.is_point_inside(point));
}

#[test]
#[should_panic(expected = "assertion failed: check_polygon_is_convex(&vertices)")]
fn test_non_convex_polygon() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 }, // Creates a concave shape
        Point { x: 2.0, y: 2.0 },
        Point { x: 0.0, y: 2.0 },
    ];
    assert!(!check_polygon_is_convex(&vertices));

    ConvexPolygon::new(&vertices); // Should panic as polygon is not convex
}


#[test]
fn test_segment_intersects_triangle() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let segment = Segment::new(
        Point { x: 0.0, y: 1.0 },
        Point { x: 2.0, y: 1.0 }
    );
    let pts = polygon.intersect_with_segment(&segment);
    assert!(pts.0.is_some());
    assert_eq!(pts.0.unwrap().x, 1.5);
    assert_eq!(pts.0.unwrap().y, 1.0);
    
    assert!(pts.1.is_some());
    assert_eq!(pts.1.unwrap().x, 0.5);
    assert_eq!(pts.1.unwrap().y, 1.0);

}

#[test]
fn test_segment_outside_polygon() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let segment = Segment::new(
        Point { x: -2.0, y: 3.0 },
        Point { x: -1.0, y: 3.0 }
    );
    let (pt1, pt2) = polygon.intersect_with_segment(&segment);
    assert!(pt1.is_none());
    assert!(pt2.is_none());
}

#[test]
fn test_segment_touches_vertex() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let segment = Segment::new(
        Point { x: 0.0, y: 0.0 },
        Point { x: -1.0, y: -1.0 },
    );
    let (pt1, pt2) = polygon.intersect_with_segment(&segment);
    
    assert!(pt1.is_some());
    assert_eq!(pt1.unwrap().x, 0.0);
    assert_eq!(pt1.unwrap().y, 0.0);
    dbg!(&pt2);
    assert!(pt2.is_none());
}

#[test]
fn test_segment_touches_edge() {
    let vertices = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let polygon = ConvexPolygon::new(&vertices);
    let segment = Segment::new(
        Point { x: 1.0, y: 0.0 },
        Point { x: -1.0, y: -1.0 },
    );
    let (pt1, pt2) = polygon.intersect_with_segment(&segment);
    assert!(pt1.is_some());
    assert_eq!(pt1.unwrap().x, 1.0);
    assert_eq!(pt1.unwrap().y, 0.0);
    assert!(pt2.is_none());
}
