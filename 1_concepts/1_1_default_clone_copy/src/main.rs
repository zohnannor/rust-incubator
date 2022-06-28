#![allow(dead_code)]

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    pub fn new(point: Point) -> Self {
        Self {
            points: vec![point],
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}

#[derive(Debug)]
struct EmptyPolyline;

impl TryFrom<Vec<Point>> for Polyline {
    type Error = EmptyPolyline;

    fn try_from(points: Vec<Point>) -> Result<Self, Self::Error> {
        if points.is_empty() {
            Err(EmptyPolyline)
        } else {
            Ok(Self { points })
        }
    }
}

fn main() {
    let point = Point::default();

    let point2 = Point { x: 1.0, y: 2.0 };

    let line = Polyline::try_from(vec![]).unwrap();

    dbg!(point);
    dbg!(point2);
    dbg!(&line, line.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyline() {
        let point = Point::default();
        let line = Polyline::new(point);
        assert_eq!(line.len(), 1);

        let line = Polyline::try_from(vec![]);

        assert!(line.is_err());
    }
}
