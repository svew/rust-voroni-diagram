
#![allow(dead_code)]

use std::cmp::Ordering;

use geometry::point::Point;

#[derive(Copy, Clone)]
pub struct IsValid {
    pub value : bool,
}

impl IsValid {
    pub fn new() -> IsValid {
        IsValid {
            value : true,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Site(Point), //A site event, containing the site it occurs
    Circle(Point, f64, u32, u32, IsValid), // A circle event. Contains circle center, radius, leaf pointer, and event id
}

impl Event {
    pub fn get_y(&self) -> f64 {
        match *self {
            Event::Site(point) => point.y(),
            Event::Circle(center, radius, _, _, _) => center.y() + radius,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other : &Event) -> bool {
        return self.get_y().eq(&other.get_y());
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        let lhs_y = self.get_y();
        let rhs_y = other.get_y();
        if lhs_y == rhs_y {
            match self {
                &Event::Site(_) => return Some(Ordering::Greater),
                &Event::Circle(_,_,_,_,_) => return Some(Ordering::Less), 
            }
        }
        return lhs_y.partial_cmp(&rhs_y);
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        return self.partial_cmp(other).unwrap_or(Ordering::Greater);
    }
}

