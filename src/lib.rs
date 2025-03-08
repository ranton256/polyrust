
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
