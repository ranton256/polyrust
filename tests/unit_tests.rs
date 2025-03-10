
use polyrust::{check_polygon_is_convex, generate_svg_from_polygons, intersect_convex_polygons, ConvexPolygon, Point, Segment};
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
#[should_panic]
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



#[test]
fn test_intersect_convex_polygons_no_intersection() {
    let vertices1 = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let vertices2 = vec![
        Point { x: 3.0, y: 3.0 },
        Point { x: 5.0, y: 3.0 },
        Point { x: 4.0, y: 5.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);
    
    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    assert!(intersection.is_empty());
}

#[test]
fn test_intersect_convex_polygons_overlap() {
    let vertices1 = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let vertices2 = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 3.0, y: 1.0 },
        Point { x: 2.0, y: 3.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);
    
    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    println!("intersection: {:?}", intersection);

    let inter_poly = ConvexPolygon::new(&intersection);
    let svg = generate_svg_from_polygons(&vec![&polygon1, &polygon2, &inter_poly], 
        &vec!["blue", "red", "green"],
        300, 200,
        None,
    );    
    println!("{}", svg); // for debugging. Open the SVG file in a browser to see the polygons
    
    assert!(!intersection.is_empty());

    // 1,1 1.5,1 1.25,1.5 1.5,1.25
    let expected_vertices = vec![
        "1,1",
        "1.5,1.0",
        "1.25,1.5",
    ];
    let pts = &inter_poly.vertices;
    assert_eq!(pts.len(), expected_vertices.len());
    for i in 0..intersection.len() {
        let n = expected_vertices.len();
        let i = i % n;
        
        assert_eq!(pts[i].x, expected_vertices[i].split(",").nth(0).unwrap().parse::<f32>().unwrap());
        assert_eq!(pts[i].y, expected_vertices[i].split(",").nth(1).unwrap().parse::<f32>().unwrap());
    }
    
}

#[test]
fn test_intersect_convex_polygons_vertex_touch() {
    let vertices1 = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let vertices2 = vec![
        Point { x: 1.0, y: 2.0 },
        Point { x: 3.0, y: 2.0 },
        Point { x: 2.0, y: 4.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);
    
    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    let inter_poly = ConvexPolygon::new(&intersection);
    let svg = generate_svg_from_polygons(&vec![&polygon1, &polygon2, &inter_poly],
        &vec!["blue", "red", "green"],
        300, 200, None);
    println!("{}", svg); // for debugging. Open the SVG file in a browser to see the polygons

    println!("inter_poly: {:?}", &inter_poly);

    assert!(!intersection.is_empty());
    let pts = &inter_poly.vertices;
    assert_eq!(pts.len(), 1);
    assert_eq!(pts[0].x, 1.0);
    assert_eq!(pts[0].y, 2.0);
}

#[test]
fn test_intersect_convex_polygons_edge_overlap() {
    let vertices1 = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
    ];
    let vertices2 = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
        Point { x: 5.0, y: 2.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);
    
    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    let inter_poly = ConvexPolygon::new(&intersection);
    let svg = generate_svg_from_polygons(&vec![&polygon1, &polygon2, &inter_poly],
        &vec!["blue", "red", "green"],
        300, 200, None);
    println!("{}", svg); // for debugging. Open the SVG file in a browser to see the polygons

    let pts = &inter_poly.vertices;
    assert_eq!(pts.len(), 3);
    assert_eq!(pts[0].x, 0.0);
    assert_eq!(pts[0].y, 0.0);
    assert_eq!(pts[1].x, 2.0);
    assert_eq!(pts[1].y, 0.0);
    assert_eq!(pts[2].x, 1.6666666);
    assert_eq!(pts[2].y, 0.6666667);
}

#[test]
fn test_generate_svg_from_polygons() {
    let vertices1 = vec![
        Point { x: 20.0, y: 10.0 },
        Point { x: 30.0, y: 2.0 },
        Point { x: 40.0, y: 10.0 },
        Point { x: 30.0, y: 20.0 },    ];
    let vertices2 = vec![
        Point { x: 10.0, y: 10.0 },
        Point { x: 30.0, y: 10.0 },
        Point { x: 20.0, y: 30.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);

    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    let inter_poly = ConvexPolygon::new(&intersection);
    let svg = generate_svg_from_polygons(&vec![&polygon1, &polygon2, &inter_poly], 
        &vec!["blue", "red", "green"],
        300, 200,
        Some((Point{x: 0.0, y: 0.0}, Point{x: 100.0, y: 100.0}))
    );
    
    assert!(!svg.is_empty());
    assert!(svg.contains("polygon"));
    let header = r#"<svg width="300" height="200" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">"#;
    let expected1 = r#"<polygon points="20,10 30,2 40,10 30,20 " fill="none" stroke="blue" stroke-width="0.1" />"#;
    let expected2 = r#"<polygon points="10,10 30,10 20,30 " fill="none" stroke="red" stroke-width="0.1" />"#;
    let expected3 = r#"<polygon points="20,10 30,10 26.666666,16.666666 " fill="none" stroke="green" stroke-width="0.1" />"#;

    let tail = r#"</svg>"#;
    let expected = format!("{}{}{}{}{}", header, expected1, expected2, expected3, tail);

    assert_eq!(expected, svg);

}


#[test]
fn test_generate_svg_from_polygons_auto_viewbox() {
    let vertices1 = vec![
        Point { x: 20.0, y: 10.0 },
        Point { x: 30.0, y: 2.0 },
        Point { x: 40.0, y: 10.0 },
        Point { x: 30.0, y: 20.0 },    ];
    let vertices2 = vec![
        Point { x: 10.0, y: 10.0 },
        Point { x: 30.0, y: 10.0 },
        Point { x: 20.0, y: 30.0 },
    ];
    let polygon1 = ConvexPolygon::new(&vertices1);
    let polygon2 = ConvexPolygon::new(&vertices2);

    let intersection = intersect_convex_polygons(&polygon1, &polygon2);
    let inter_poly = ConvexPolygon::new(&intersection);
    let svg = generate_svg_from_polygons(&vec![&polygon1, &polygon2, &inter_poly], 
        &vec!["blue", "red", "green"],
        48, 36,
        None
    );
    
    assert!(!svg.is_empty());
    assert!(svg.contains("polygon"));
    let header = r#"<svg width="48" height="36" xmlns="http://www.w3.org/2000/svg" viewBox="10 2 30 28">"#;
    let expected1 = r#"<polygon points="20,10 30,2 40,10 30,20 " fill="none" stroke="blue" stroke-width="0.1" />"#;
    let expected2 = r#"<polygon points="10,10 30,10 20,30 " fill="none" stroke="red" stroke-width="0.1" />"#;
    let expected3 = r#"<polygon points="20,10 30,10 26.666666,16.666666 " fill="none" stroke="green" stroke-width="0.1" />"#;

    let tail = r#"</svg>"#;
    let expected = format!("{}{}{}{}{}", header, expected1, expected2, expected3, tail);

    println!("{}", svg);
    assert_eq!(expected, svg);

}


// #[test]
// fn test_intersection_large_non_convex_polygon() {
//     let vertices1 = vec![
//         Point { x: 50.0, y: 150.0 },
//         Point { x: 200.0, y: 50.0 },
//         Point { x: 350.0, y: 150.0 },
//         Point { x: 350.0, y: 300.0 },
//         Point { x: 250.0, y: 300.0 },
//         Point { x: 200.0, y: 250.0 },
//         Point { x: 150.0, y: 350.0 },
//         Point { x: 100.0, y: 250.0 },
//         Point { x: 100.0, y: 200.0 },    ];
//     let vertices2 = vec![
//         Point { x: 100.0, y: 100.0 },
//         Point { x: 300.0, y: 100.0 },
//         Point { x: 300.0, y: 300.0 },
//         Point { x: 100.0, y: 300.0 },   
//     ];
//     let expected_intersection = vec![
//         Point { x: 100.00000, y: 116.66667 },
//         Point { x: 125.00000, y: 100.00000 },
//         Point { x: 275.00000, y: 100.00000 },
//         Point { x: 300.00000, y: 116.66667 },
//         Point { x: 300.00000, y: 300.00000 },
//         Point { x: 250.00000, y: 300.00000 },
//         Point { x: 200.00000, y: 250.00000 },
//         Point { x: 175.00000, y: 300.00000 },
//         Point { x: 125.00000, y: 300.00000 },
//         Point { x: 100.00000, y: 250.00000 }    ];
//     let polygon1 = ConvexPolygon::new(&vertices1);
//     let polygon2 = ConvexPolygon::new(&vertices2);

//     let intersection = intersect_convex_polygons(&polygon1, &polygon2);
//     assert!(!intersection.is_empty());
//     assert_eq!(intersection.len(), expected_intersection.len());
//     for i in 0..intersection.len() {
//         assert!((intersection[i].x - expected_intersection[i].x).abs() < 1e-5);
//         assert!((intersection[i].y - expected_intersection[i].y).abs() < 1e-5);    
//     }
// }
