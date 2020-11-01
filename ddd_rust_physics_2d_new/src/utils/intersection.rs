use crate::containers::entity::Entity;
use crate::utils::point::Point;

#[derive(Debug, PartialEq, Clone)]
pub enum IntersectionResult {
    Intersection,
    NoIntersection,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    pub result_type: IntersectionResult,
    pub result: Option<(Point, Point)>,
}

pub fn circle_intersection(c0: Entity, c1: Entity) -> Intersection {
    let r0 = c0.entity.circle.unwrap().get_radius();
    let r1 = c1.entity.circle.unwrap().get_radius();

    let c0_position = c0.physical.get_position();
    let c1_position = c1.physical.get_position();

    let (c0x, c0y) = c0_position.get_tuple();
    let (c1x, c1y) = c1_position.get_tuple();

    // x,y-distances between centers
    let dx = c1x - c0x;
    let dy = c1y - c0y;

    // straight-line distance between centers
    let d = (dx.powi(2) + dy.powi(2)).sqrt();

    if d > (r0 + r1) || d <= 0.0 {
        // one circle is contained by the other circle
        // or the circles are coincident
        Intersection {
            result_type: IntersectionResult::NoIntersection,
            result: None,
        }
    } else {
        // distance from (c0x, c0y) to intersection of line through centers and line through intersection points
        let a = (r0.powi(2) - r1.powi(2) + d.powi(2)) / (2.0 * d);

        // coordinates of above intersection point
        let line_intersect = Point::new(c0x + (dx * a / d), c0y + (dx * a / d));

        // distance from p_intersect to circle intersection points
        let h = (r0.powi(2) - a.powi(2)).sqrt();

        // offset vector of circle intersection points from line_intersect
        let offset = Point::new(-dy * (h / d), dx * (h / d));

        // determine points of intersection on circles
        let p0 = Point::new(line_intersect.x + offset.x, line_intersect.y + offset.y);
        let p1 = Point::new(line_intersect.x - offset.x, line_intersect.y - offset.y);

        Intersection {
            result_type: IntersectionResult::Intersection,
            result: Some((p0, p1)),
        }
    }
}
