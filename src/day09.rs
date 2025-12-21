use std::collections::BTreeMap;

use crate::Aoc;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/09.in");

pub struct Day09;

impl Aoc for Day09 {
    fn run(&self) -> (String, String) {
        let (pt1, pt2) = run_on_input(INPUT);
        (pt1.to_string(), pt2.to_string())
    }
}

fn run_on_input(input: &str) -> (i64, i64) {
    let coordinates: Vec<Point> = input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut rectangles: Vec<Rectangle> = coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rectangle::new(a.clone(), b.clone()))
        .collect();

    rectangles.sort_unstable_by_key(|rect| rect.area());
    let largest_area = rectangles.last().unwrap().area();

    let mut bounding_polygon = BoundingPolygon::new(coordinates.clone());

    while let Some(rectangle) = rectangles.pop() {
        // A rectangle is a valid answer for part 2 if it's entirely contained in the bounding
        // polygon.  A rectangle is definitely in the bounding polygon if:
        //   1. All corners of the rectangle are inside or on the boundary
        //   2. None of the boundary lines intersect the lines of the rectangle
        //
        // It's also possible for a rectangle to be a valid answer if the bounding polygon intrudes
        // on the rectangle, but the intrusion is only of width 2 (and therefore there are no grid
        // points in the intrusion that are not part of the bounding line). The solution assumes
        // that this never happens.
        let rectangle_is_inside_boundary = rectangle.corners().iter().all(|corner| {
            matches!(
                bounding_polygon.contains(corner),
                Bounding::Inside | Bounding::OnBoundary
            )
        });

        let any_boundary_lines_intersect_rectangle =
            // Weirdly, having the rectangle line segments as the outer iteration and the bounding
            // polygon as the inner seems to perform significantly better here. I have no idea
            // why...
            rectangle.iter_line_segments().any(|rectangle_seg| {
                bounding_polygon.iter_line_segments().any(|boundary_seg| {
                    match rectangle_seg.intersects(&boundary_seg) {
                        Intersection::Intersecting => true,
                        Intersection::NotIntersecting => false,
                        Intersection::PerpendicularTouching(_point) => {
                            // For now, we're not going to consider this as an intersection. We may
                            // need to revisit this, because there is a case where two line segments
                            // both touch the rectangle from opposite sides, so there is actually
                            // an intersection but this would not count it as one. But for now we're
                            // not considering this edge case...
                            false
                        }
                    }
                })
            });

        if rectangle_is_inside_boundary && !any_boundary_lines_intersect_rectangle {
            return (largest_area, rectangle.area());
        }
    }

    panic!("No rectangles match the search criteria...");
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Rectangle {
    corner_a: Point,
    corner_b: Point,
    corner_c: Point,
    corner_d: Point,
}

impl Rectangle {
    fn new(corner_a: Point, corner_b: Point) -> Self {
        let corner_c = Point::new(corner_a.x, corner_b.y);
        let corner_d = Point::new(corner_b.x, corner_a.y);
        Self {
            corner_a,
            corner_b,
            corner_c,
            corner_d,
        }
    }

    fn area(&self) -> i64 {
        let xdiff = self.corner_a.x - self.corner_b.x;
        let ydiff = self.corner_a.y - self.corner_b.y;
        (xdiff.abs() + 1) * (ydiff.abs() + 1)
    }

    fn corners(&self) -> [&Point; 4] {
        // Note the ordering of corners is important here, so that the line segments drawn between
        // each consecutive pair of corners are the sides of the rectangle.
        [
            &self.corner_a,
            &self.corner_c,
            &self.corner_b,
            &self.corner_d,
        ]
    }

    fn iter_line_segments(&self) -> impl Iterator<Item = LineSegment<'_>> {
        self.corners()
            .into_iter()
            .circular_tuple_windows()
            .map(|(a, b)| LineSegment::new(a, b))
    }
}

#[derive(Debug, Clone)]
struct LineSegment<'a> {
    a: &'a Point,
    b: &'a Point,
}

impl<'a> LineSegment<'a> {
    fn new(a: &'a Point, b: &'a Point) -> Self {
        Self { a, b }
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn intersects(&self, other: &LineSegment) -> Intersection {
        if self.is_vertical() {
            if other.is_vertical() {
                Intersection::NotIntersecting
            } else {
                let x_range_overlaps = (other.a.x > self.a.x) != (other.b.x > self.a.x);
                let y_range_overlaps = (self.a.y > other.a.y) != (self.b.y > other.a.y);
                if !x_range_overlaps || !y_range_overlaps {
                    Intersection::NotIntersecting
                } else if self.a.y == other.a.y {
                    Intersection::PerpendicularTouching(self.a.clone())
                } else if self.b.y == other.a.y {
                    Intersection::PerpendicularTouching(self.b.clone())
                } else if other.a.x == self.a.x {
                    Intersection::PerpendicularTouching(other.a.clone())
                } else if other.b.x == self.a.x {
                    Intersection::PerpendicularTouching(other.b.clone())
                } else {
                    Intersection::Intersecting
                }
            }
        } else if other.is_horizontal() {
            Intersection::NotIntersecting
        } else {
            let y_range_overlaps = (other.a.y > self.a.y) != (other.b.y > self.a.y);
            let x_range_overlaps = (self.a.x > other.a.x) != (self.b.x > other.a.x);
            if !y_range_overlaps || !x_range_overlaps {
                Intersection::NotIntersecting
            } else if self.a.x == other.a.x {
                Intersection::PerpendicularTouching(self.a.clone())
            } else if self.b.x == other.a.x {
                Intersection::PerpendicularTouching(self.b.clone())
            } else if other.a.y == self.a.y {
                Intersection::PerpendicularTouching(other.a.clone())
            } else if other.b.y == self.a.y {
                Intersection::PerpendicularTouching(other.b.clone())
            } else {
                Intersection::Intersecting
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Intersection {
    Intersecting,
    NotIntersecting,
    PerpendicularTouching(Point),
}

struct BoundingPolygon {
    points: Vec<Point>,
    cache: BTreeMap<Point, Bounding>,
}

impl BoundingPolygon {
    /// Points are ordered (e.g. each consecutive point is joined by a LineSegment)
    fn new(points: Vec<Point>) -> Self {
        let polygon = Self {
            points,
            cache: BTreeMap::new(),
        };

        // Manually inspecting the input, we see that all line segments are either vertical or
        // horizontal. Assert that this is actually the case, as we'll use this assumption later.
        assert!(
            polygon
                .iter_line_segments()
                .all(|seg| seg.is_vertical() || seg.is_horizontal())
        );

        polygon
    }

    fn iter_line_segments(&self) -> impl Iterator<Item = LineSegment<'_>> {
        self.points
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| LineSegment::new(a, b))
    }

    fn contains(&mut self, point: &Point) -> Bounding {
        if let Some(bounding) = self.cache.get(point) {
            return *bounding;
        }

        let bounding = self.contains_inner(point);
        self.cache.insert(point.clone(), bounding);
        bounding
    }

    fn contains_inner(&self, point: &Point) -> Bounding {
        // Ray casting algorithm to see if a point lies within the polygon. A point is within the
        // polygon if a horizontal line drawn from the point to infinity crosses the bounding line
        // of the polygon an odd number of times.
        let mut crossing_count = 0;
        for seg in self.iter_line_segments() {
            if seg.is_horizontal() {
                let x_range_intersects = (seg.a.x > point.x) != (seg.b.x > point.x);
                if point.y == seg.a.y && x_range_intersects {
                    return Bounding::OnBoundary;
                }
                continue;
            }

            let y_range_intersects = (seg.a.y > point.y) != (seg.b.y > point.y);
            let x_to_left = point.x < seg.a.x;

            if point.x == seg.a.x && y_range_intersects {
                return Bounding::OnBoundary;
            }

            if y_range_intersects && x_to_left {
                crossing_count += 1;
            }
        }

        if crossing_count % 2 == 0 {
            Bounding::Outside
        } else {
            Bounding::Inside
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bounding {
    Inside,
    Outside,
    OnBoundary,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../inputs/09.ex");

    #[test]
    fn test_example() {
        let (pt1, pt2) = run_on_input(EXAMPLE_INPUT);
        assert_eq!(50, pt1);
        assert_eq!(24, pt2);
    }

    #[test]
    fn test_area_between_corners() {
        let a = Point::new(2, 5);
        let b = Point::new(11, 1);
        let rectangle = Rectangle::new(a, b);
        assert_eq!(50, rectangle.area());
    }

    #[test]
    fn test_bounding_polygon() {
        let polygon = BoundingPolygon::new(vec![
            Point::new(0, 0),
            Point::new(5, 0),
            Point::new(5, 5),
            Point::new(0, 5),
        ]);

        assert_eq!(Bounding::Inside, polygon.contains(&Point::new(1, 1)));
        assert_eq!(Bounding::OnBoundary, polygon.contains(&Point::new(5, 3)));
        assert_eq!(Bounding::Outside, polygon.contains(&Point::new(1, 6)));
    }

    #[test]
    fn test_bounding_polygon_weird_shape() {
        // 012345678901234     012345678901234
        // 1.+----------+.     1.+----------+.
        // 2.|..+---+...|.     2.|..+---+...|.
        // 3.|..|...|...|.     3.|..|...|...|.
        // 4.+--+...+---+.     4.+--+...+---+.
        // 5..............     5..............
        let polygon = BoundingPolygon::new(vec![
            Point::new(2, 1),
            Point::new(13, 1),
            Point::new(13, 4),
            Point::new(9, 4),
            Point::new(9, 2),
            Point::new(5, 2),
            Point::new(5, 4),
            Point::new(2, 4),
        ]);

        assert_eq!(Bounding::Inside, polygon.contains(&Point::new(3, 3)));
        assert_eq!(Bounding::OnBoundary, polygon.contains(&Point::new(9, 4)));
        assert_eq!(Bounding::Outside, polygon.contains(&Point::new(7, 3)));
    }

    #[test]
    fn test_bounding_polygon_example_input() {
        let polygon = BoundingPolygon::new(vec![
            Point::new(7, 1),
            Point::new(11, 1),
            Point::new(11, 7),
            Point::new(9, 7),
            Point::new(9, 5),
            Point::new(2, 5),
            Point::new(2, 3),
            Point::new(7, 3),
        ]);

        assert_eq!(Bounding::Outside, polygon.contains(&Point::new(2, 7)));
    }

    #[test]
    fn test_line_segment_intersecting() {
        let pta = Point::new(0, 3);
        let ptb = Point::new(5, 3);
        let ptc = Point::new(3, 0);
        let ptd = Point::new(3, 5);
        let a = LineSegment::new(&pta, &ptb);
        let b = LineSegment::new(&ptc, &ptd);
        assert_eq!(Intersection::Intersecting, a.intersects(&b));
        assert_eq!(Intersection::Intersecting, b.intersects(&a));

        let pte = Point::new(3, 3);
        let c = LineSegment::new(&pte, &ptd);
        assert_eq!(
            Intersection::PerpendicularTouching(Point::new(3, 3)),
            a.intersects(&c)
        );
        assert_eq!(
            Intersection::PerpendicularTouching(Point::new(3, 3)),
            c.intersects(&a)
        );
    }

    #[test]
    fn test_line_segment_intersecting_from_example() {
        let pta = Point { x: 7, y: 1 };
        let ptb = Point { x: 11, y: 1 };
        let a = LineSegment { a: &pta, b: &ptb };
        let ptc = Point { x: 9, y: 5 };
        let ptd = Point { x: 9, y: 3 };
        let b = LineSegment { a: &ptc, b: &ptd };

        // Should not intersect because the y value of segment a lies completely outside the y range
        // of segment b.
        assert_eq!(Intersection::NotIntersecting, a.intersects(&b));
        assert_eq!(Intersection::NotIntersecting, b.intersects(&a));
    }
}
