
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
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

impl ConvexPolygon {
    pub fn new(vertices: &Vec<Point>) -> ConvexPolygon {
        assert!(check_polygon_is_convex(&vertices));
        ConvexPolygon { vertices: vertices.clone() }
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
