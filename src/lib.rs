use std::iter::zip;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

fn order_vertices_clockwise(vertices: &mut Vec<Point>) {
    let centroid = Point {
        x: vertices.iter().map(|p| p.x).sum::<f32>() / vertices.len() as f32,
        y: vertices.iter().map(|p| p.y).sum::<f32>() / vertices.len() as f32,
    };
    vertices.sort_by(|a, b| {
        let angle_a = (a.y - centroid.y).atan2(a.x - centroid.x);
        let angle_b = (b.y - centroid.y).atan2(b.x - centroid.x);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConvexPolygon {
    pub vertices: Vec<Point>,
}

fn is_left(p0: Point, p1: Point, p2: Point) -> f32 {
    (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)
}

pub fn check_polygon_is_convex(vertices: &Vec<Point>) -> bool {
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    let mut is_convex = true;
    for i in 0..n {
        let p0 = vertices[i];
        let p1 = vertices[(i + 1) % n];
        let p2 = vertices[(i + 2) % n];
        if is_left(p0, p1, p2) < 0.0 {
            is_convex = false;
            break;
        }
    }
    is_convex
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

impl Line {
    pub fn new(a: f32, b: f32, c: f32) -> Line {
        Line { a, b, c }
    }
    pub fn from_points(p1: Point, p2: Point) -> Line {
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;
        let c = a * p1.x + b * p1.y;
        Line::new(a, b, c)
    }
}

impl Segment {
    pub fn new(p1: Point, p2: Point) -> Segment {
        Segment { p1, p2 }
    }
}

fn value_between(x: f32, min: f32, max: f32) -> bool {
    x >= min && x <= max
}

fn value_between_bidirectional(x: f32, a: f32, b: f32) -> bool {
    value_between(x, a, b) || value_between(x, b, a)
}

fn on_segment(p: Point, s: &Segment) -> bool {
    let (q , r) = (s.p1, s.p2);
    value_between_bidirectional(p.x, q.x, r.x) && value_between_bidirectional(p.y, q.y, r.y)
}

pub fn intersect_line_segments(seg_one: &Segment, seg_two: &Segment) -> Option<Point> {
    let line_one = Line::from_points(seg_one.p1, seg_one.p2);
    let line_two = Line::from_points(seg_two.p1, seg_two.p2);

    let a1 = line_one.a;
    let b1 = line_one.b;
    let c1 = line_one.c;
    
    let a2 = line_two.a;
    let b2 = line_two.b;
    let c2 = line_two.c;

    let det = a1 * b2 - a2 * b1;

    if det == 0.0 {
        // Lines are parallel
        return None
    } else{
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        let p = Point{x, y};
        if on_segment(Point{x, y}, seg_one) && on_segment(p, seg_two) {
            return Some(p)
        }

        return None
    }
}


impl ConvexPolygon {
    pub fn new(vertices: &Vec<Point>) -> ConvexPolygon {
        assert!(check_polygon_is_convex(vertices));
        let mut vertices_copy = dedup_vertices(vertices);
        order_vertices_clockwise(&mut vertices_copy);
        ConvexPolygon { vertices: vertices_copy }
    }

    pub fn is_point_inside(&self, p: Point) -> bool {
        let mut winding_number = 0;
        let n = self.vertices.len();
        for i in 0..n {
            let v1 = self.vertices[i];
            let v2 = self.vertices[(i + 1) % n];
            if v1.y <= p.y {
                if v2.y > p.y && is_left(v1, v2, p) > 0.0 {
                    winding_number += 1;
                }
            } else {
                if v2.y <= p.y && is_left(v1, v2, p) < 0.0 {
                    winding_number -= 1;
                }
            }
        }
        winding_number != 0
    }

    pub fn intersect_with_segment(&self, segment: &Segment) -> (Option<Point>, Option<Point>) {
        let mut intersections = Vec::new();
        let n = self.vertices.len();
        for i in 0..n {
            let v1 = self.vertices[i];
            let v2 = self.vertices[(i + 1) % n];
            let seg = Segment::new(v1, v2);
            if let Some(intersection) = intersect_line_segments(&seg, segment) {
                intersections.push(intersection);
            }
        }
        let intersections = dedup_vertices(&intersections);
        assert!(intersections.len() <= 2);
        
        if intersections.is_empty() {
            (None, None)
        } else if intersections.len() == 1 || intersections[0] == intersections[1] {
            (Some(intersections[0]), None)
        } else {
            (Some(intersections[0]), Some(intersections[1]))
        }
    }
}

fn dedup_vertices(vertices: &Vec<Point>) -> Vec<Point> {
    let mut vertices_copy: Vec<Point> = vec![];
    for p in vertices {
        if !vertices_copy.contains(p) {
            vertices_copy.push(*p);
        }
    }
    vertices_copy
}


pub fn intersect_convex_polygons(poly_one: &ConvexPolygon, poly_two: &ConvexPolygon) -> Vec<Point> {
    let mut intersections = Vec::new();
    let n1 = poly_one.vertices.len();
    let n2 = poly_two.vertices.len();
    for i in 0..n1 {
        if poly_two.is_point_inside(poly_one.vertices[i]) {
            intersections.push(poly_one.vertices[i]);
        }
        let v1 = poly_one.vertices[i];
        let v2 = poly_one.vertices[(i + 1) % n1];
        let seg = Segment::new(v1, v2);
        let (p1, p2) = poly_two.intersect_with_segment(&seg);
        if let Some(p) = p1 {
            intersections.push(p);
        }
        if let Some(p) = p2 {
            intersections.push(p);
        }
    }
    for i in 0..n2 {
        if poly_one.is_point_inside(poly_two.vertices[i]) {
            intersections.push(poly_two.vertices[i]);
        }
        let v1 = poly_two.vertices[i];
        let v2 = poly_two.vertices[(i + 1) % n2];
        let seg = Segment::new(v1, v2);
        let (p1, p2) = poly_one.intersect_with_segment(&seg);
        if let Some(p) = p1 {
            intersections.push(p);
        }
        if let Some(p) = p2 {
            intersections.push(p);
        }
    }
    intersections
}


pub fn generate_svg_from_polygons(polygons: &Vec<&ConvexPolygon>, colors: &Vec<&str>, width: u32, height: u32, view_box: Option<(Point,Point)>) -> String {
    let mut min_x: f32;
    let mut min_y: f32;
    let mut max_x: f32;
    let mut max_y: f32;

    if view_box.is_none() {
        min_x = std::f32::MAX;
        min_y = std::f32::MAX;
        max_x = std::f32::MIN;
        max_y = std::f32::MIN;
        for polygon in polygons {
            for vertex in &polygon.vertices {
                min_x = min_x.min(vertex.x);
                min_y = min_y.min(vertex.y);
                max_x = max_x.max(vertex.x);
                max_y = max_y.max(vertex.y);
            }
        }
    } else {
        let (min, max) = view_box.unwrap();
        min_x = min.x;
        min_y = min.y;
        max_x = max.x;
        max_y = max.y;
    };
    let view_width = max_x - min_x;
    let view_height = max_y - min_y;

    let mut svg = format!("<svg width=\"{width}\" height=\"{height}\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{min_x} {min_y} {view_width} {view_height}\">");
    for (polygon, color) in zip(polygons, colors) {
        let mut points = String::new();
        for vertex in &polygon.vertices {
            points.push_str(&format!("{},{} ", vertex.x, vertex.y));
        }
        svg.push_str(&format!("<polygon points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.1\" />", points, color));
    }
    svg.push_str("</svg>");
    svg
}
