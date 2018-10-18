
use geometry::point::Point;

#[derive(Clone, Default, PartialEq)]
pub struct Bound {
    min_point : Option<Point>,
    max_point : Option<Point>,
}

impl Bound {
    pub fn new() -> Bound {
        Bound {
            min_point : None,
            max_point : None,
        }
    }

    pub fn update(&mut self, point: &Point) {
        if let Some(min) = self.min_point {
            self.min_point = Some(Point::new(
                {if point.x() < min.x() {point.x()} else {min.x()}},
                {if point.y() < min.y() {point.y()} else {min.y()}}));
        } else {
            self.min_point = Some(*point);
        }

        if let Some(max) = self.max_point {
            self.max_point = Some(Point::new(
                {if point.x() > max.x() {point.x()} else {max.x()}},
                {if point.y() > max.y() {point.y()} else {max.y()}}));
        } else {
            self.max_point = Some(*point);
        }
    }

    pub fn get_top(&self) -> Option<f64> {
        self.min_point.and_then(|min| self.margin().and_then(|marg| Some(min.y() - marg)))
    }

    pub fn get_bottom(&self) -> Option<f64> {
        self.max_point.and_then(|max| self.margin().and_then(|marg| Some(max.y() + marg)))
    }

    pub fn get_left(&self) -> Option<f64> {
        self.min_point.and_then(|min| self.margin().and_then(|marg| Some(min.x() - marg)))
    }
    
    pub fn get_right(&self) -> Option<f64> {
        self.max_point.and_then(|max| self.margin().and_then(|marg| Some(max.x() + marg)))
    }
	
	pub fn get_max(&self) -> Option<Point> {
		self.max_point
	}
	
	pub fn get_min(&self) -> Option<Point> {
		self.min_point
	}

    fn margin(&self) -> Option<f64> {
        if let Some(min) = self.min_point {
            if let Some(max) = self.max_point {
                let diff = max - min;
                let larger = if diff.x() > diff.y() {diff.x()} else {diff.y()};
                return Some(larger/5.0 + 1.0); 
            }
        }
        return None;
    }

    pub fn find_bound_intersection(&self, site1 : &Point, site2 : &Point) -> Option<Point> {
        if self.get_right().is_none() {
            return None;
        }
        return None;
/*
        let mid = (*site2 - *site1)*0.5 + *site1;
        let ray = Point::new(
            site2.y() - site1.y(),
            site1.x() - site2.x());

        let x_bound = if ray.x() > 0 { 
            self.get_right.unwrap() 
        } else if ray.x() < 0 {
            self.get_left.unwrap()
        } else {
            if ray.y() > 0 {
                return Some(Point::new(mid.x(), self.get_bottom.unwrap()));
            } else {
                return Some(Point::new(mid.x(), self.get_top.unwrap()));
            }
        };
        let y_bound = if ray.y() > 0 { 
            self.get_bottom.unwrap() 
        } else if ray.y() < 0 {
            self.get_top.unwrap()
        } else {
            if ray.x() > 0 {
                return Some(Point::new(self.get_right.unwrap(), mid.y()));
            } else {
                return Some(Point::new(self.get_left.unwrap(), mid.y()));
            }
        };
        let a = Point::new(x_bound, (x_bound - mid.x()))
*/
    }
    
    fn closer_point(x : &Point, a : &Point, b : &Point) -> Point {
        let x_a = *x - *a;
        let x_b = *x - *b;
        let x_a_sq = x_a.x()*x_a.x() + x_a.y()*x_a.y();
        let x_b_sq = x_b.x()*x_b.x() + x_b.y()*x_b.y();
        if x_a_sq > x_b_sq { 
            b.clone()
        } else { 
            a.clone()
        }
    }
}