
use geometry::point::Point;

pub fn get_circle_center(triple : &(Point, Point, Point)) -> Option<Point> {
	let a = triple.1.x() - triple.0.x();
	let b = triple.1.y() - triple.0.y();
	let c = triple.2.x() - triple.0.x();
	let d = triple.2.y() - triple.0.y();
	let e = (&a)*(triple.0.x() + triple.1.x()) + (&b)*(triple.0.y() + triple.1.y());
	let f = (&c)*(triple.0.x() + triple.2.x()) + (&d)*(triple.0.y() + triple.2.y());
	let g = 2.0*((&a)*(triple.2.y() - triple.1.y()) - (&b)*(triple.2.x() - triple.1.x()));
	
	if g == 0.0 { return None }

	return Some(Point::new(
		(d*e-b*f)/g,
		(a*f-c*e)/g,
	));
}

pub fn get_parabola_intersection_x(focus1 : &Point, focus2 : &Point, line_y : f64) -> f64 {

    let ax = focus1.x();
    let bx = focus2.x();
    let ay = focus1.y();
    let by = focus2.y();

    // shift frames
    let bx_s = bx - ax;
    let ay_s = ay - line_y;
    let by_s = by - line_y;

    let discrim = ay_s * by_s * ((ay_s - by_s) * (ay_s - by_s) + bx_s * bx_s);
    let numer = ay_s * bx_s - discrim.sqrt();
    let denom = ay_s - by_s;

    let mut x_bp = if denom != 0.0 {
        numer / denom
    } else {
        bx_s / 2.
    };
    x_bp += ax; // shift back to original frame

    return x_bp;
}

pub fn get_parabola_y(focus : &Point, line_y : f64, x : f64) -> f64 {
	let parabola = get_parabola(focus, line_y);
	return parabola.0 * x * x + parabola.1 * x + parabola.2;
}

pub fn get_parabola(focus : &Point, line_y : f64) -> (f64, f64, f64) {
	
	let dp : f64 = 2.0*(focus.y() - line_y);
	let a : f64 = 1.0/dp;
	let b : f64 = -2.0*focus.x()/dp;
	let c : f64 = (focus.x()*focus.x() + focus.y()*focus.y() - line_y*line_y)/dp;
	return (a, b, c);
}

pub fn is_clockwise(triple : &(Point, Point, Point)) -> bool {
	if triple.0 == triple.1 || triple.1 == triple.2 || triple.0 == triple.2 {
		return false;
	}
	
    let (a, b, c) = *triple;
    let ax = a.x();
    let ay = a.y();
    let bx = b.x();
    let by = b.y();
    let cx = c.x();
    let cy = c.y();

    return (ay - by) * (bx - cx) > (by - cy) * (ax - bx)
}

pub fn get_distance(p1 : &Point, p2 : &Point) -> f64 {
	let v = *p2 - *p1;
    return (v.x()*v.x() + v.y()*v.y()).sqrt();
}
