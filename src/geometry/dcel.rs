
extern crate glium;

use std::fmt;

use geometry::bound::Bound;
use geometry::edge::Edge;
use geometry::face::Face;
use geometry::point::Point;
use geometry::vertex::Vertex;
use graphics::display::OpenglVertex;

pub type VertexPtr = u32;
pub type EdgePtr = u32;
pub type FacePtr = u32;

#[derive(Clone, Default, PartialEq)]
pub struct DoublyConnectedEdgeList {
	index_vertex : u32,
	index_edge : u32,
	index_face : u32,
	pub vertices : Vec<Vertex>,
	pub edges : Vec<Edge>,
	pub faces : Vec<Face>,
	bounding_box : Bound,
}

impl DoublyConnectedEdgeList {
	pub fn new() -> DoublyConnectedEdgeList {
		DoublyConnectedEdgeList {
			index_vertex : 0,
			index_edge : 0,
			index_face : 0,
			vertices : Vec::new(),
			edges : Vec::new(),
			faces : Vec::new(),
			bounding_box : Bound::new(),
		}
	}
	pub fn new_vertex(&mut self, point : &Point) -> VertexPtr {
		for vertex in &self.vertices {
			if vertex.point == *point {
				return vertex.index();
			}
		}

		let index = self.index_vertex;
		let new_vertex = Vertex::new(index, *point);
		self.bounding_box.update(point);
		self.vertices.push(new_vertex);
		self.index_vertex += 1;
		return index;
	}
	pub fn new_edge(&mut self, from : Option<u32>) -> EdgePtr {
		let index = self.index_edge;
		self.edges.push(Edge::new(index, from));
		self.index_edge += 1;
		return index;
	}
	pub fn new_face(&mut self, site : Option<VertexPtr>) -> FacePtr {
		let index = self.index_face;
		self.faces.push(Face::new(index, site));
		self.index_face += 1;
		return index;
	}
	pub fn new_dual_edges(&mut self, e1_origin : Option<VertexPtr>, 
			e2_origin : Option<VertexPtr>) -> (EdgePtr, EdgePtr) {

		let index1 = self.new_edge(e1_origin);
		let index2 = self.new_edge(e2_origin);
		self.get_edge(index1).twin = Some(index2);
		self.get_edge(index2).twin = Some(index1);

		(index1, index2)
	}
	pub fn get_vertex(&mut self, index : u32) -> &mut Vertex {
		return self.vertices.get_mut(index as usize).unwrap();
	}
	pub fn get_edge(&mut self, index : u32) -> &mut Edge {
		return self.edges.get_mut(index as usize).unwrap();
	}	
	pub fn get_face(&mut self, index : u32) -> &mut Face {
		return self.faces.get_mut(index as usize).unwrap();
	}
	pub fn get_imm_vertex(&self, index : u32) -> &Vertex {
		return self.vertices.get(index as usize).unwrap();
	}
	pub fn get_imm_edge(&self, index : u32) -> &Edge {
		return self.edges.get(index as usize).unwrap();
	}	
	pub fn get_imm_face(&self, index : u32) -> &Face {
		return self.faces.get(index as usize).unwrap();
	}
	pub fn get_edge_tuple(&self, edge_ptr : EdgePtr) -> Option<(VertexPtr, VertexPtr)> {
		let twin_ptr_option;
		let origin_ptr_option;
		if let Some(ref edge) = self.edges.get(edge_ptr as usize) {
			twin_ptr_option = edge.twin;
			origin_ptr_option = edge.origin;
		} else {
			return None;
		}

		let twin_origin_ptr_option;
		if let Some(twin_ptr) = twin_ptr_option {
			if let Some(ref twin) = self.edges.get(twin_ptr as usize) {
				twin_origin_ptr_option = twin.origin;
			} else {
				return None;
			}
		} else {
			return None;
		}

		if let Some(vertex1_ptr) = origin_ptr_option {
			if let Some(vertex2_ptr) = twin_origin_ptr_option {
				return Some((vertex1_ptr, vertex2_ptr))
			}
		}
		return None;
	}
	pub fn get_opengl_vertices(&self) -> Vec<OpenglVertex> {

		let mut out = Vec::new();

		for edge in &self.edges {
			if let Some(origin_ptr) = edge.origin {
				if let Some(twin_ptr) = edge.twin {
					if let Some(face_ptr) = edge.incident_face {
						if let Some(twin_origin_ptr) = self.get_imm_edge(twin_ptr).origin {
							let color = self.get_imm_face(face_ptr).color();
							let a = self.get_imm_vertex(origin_ptr).point;
							let b = self.get_imm_vertex(twin_origin_ptr).point;
							
							let a_gl = self.opengl_point_shift(&a);
							let b_gl = self.opengl_point_shift(&b);
							let unit_vector = (b - a).unit();
							let unit_shift_90 = Point::new(-unit_vector.y(), unit_vector.x());
							let offset = unit_shift_90 * 0.005;
							let c_gl = a_gl + offset;
							let d_gl = b_gl + offset;

							let a_vertex = OpenglVertex {
								position : [a_gl.x() as f32, a_gl.y() as f32],
								color : color,};
							let b_vertex = OpenglVertex {
								position : [b_gl.x() as f32, b_gl.y() as f32],
								color : color,};
							let c_vertex = OpenglVertex {
								position : [c_gl.x() as f32, c_gl.y() as f32],
								color : color,};
							let d_vertex = OpenglVertex {
								position : [d_gl.x() as f32, d_gl.y() as f32],
								color : color,};

							out.push(a_vertex.clone());
							out.push(b_vertex.clone());
							out.push(c_vertex.clone());
							out.push(b_vertex.clone());
							out.push(c_vertex.clone());
							out.push(d_vertex.clone());
						}
					}
				}
			}
		}
		out
	}
	fn opengl_point_shift(&self, a : &Point) -> Point {

		//let mult_x = 2.0/(self.bounding_box.get_right().unwrap() - self.bounding_box.get_left().unwrap());
		//let mult_y = 2.0/(self.bounding_box.get_bottom().unwrap() - self.bounding_box.get_top().unwrap());
		let mult_x = 1.0/20.0;
		let mult_y = 1.0/20.0;
		Point::new(a.x() * mult_x, a.y() * mult_y)
	}
}

impl fmt::Debug for DoublyConnectedEdgeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let mut s = String::from("Doubly Connected Edge List\n\n");
		s.push_str("\tVertices (index, location, incident edge)\n");

		for vertex in &self.vertices {
			/*
			let edge = match vertex.incident_edge {
				Some(e) => match self.get_edge_tuple(e) {
					Some((v1, v2)) => format!("e{},{}", v1, v2),
					None => String::from("nil"),
				}, 
				None => String::from("nil"),
			};
			*/
			let edge = match vertex.incident_edge {
				Some(e) => format!("e{}", e),
				_ => String::from("nil"),
			};
			let is_site = if vertex.is_site {
				String::from("(Site)")
			} else {
				String::from("")
			};

			s.push_str(format!("\tv{} ({:.2}, {:.2}) {} {}\n", vertex.index(), vertex.point.x(), vertex.point.y(), edge, is_site).as_str());
		}
		
		s.push_str("\n\tEdges (index, origin, twin, next, prev, face)\n");

		for edge in &self.edges {
			/*
			let edge_str = match self.get_edge_tuple(*index) {
				Some((v1, v2)) => format!("{},{}", v1, v2),
				_ => String::from("{nil}"),};
			*/
			let edge_str = format!("{}", edge.index());

			let origin = match edge.origin {
				Some(origin_ptr) => format!("v{}", origin_ptr),
				_ => String::from("nil"),
			};

			let next = match edge.next {
				Some(ptr) => format!("e{}", ptr),
				_ => String::from("nil"),
			};

			let prev = match edge.prev {
				Some(ptr) => format!("e{}", ptr),
				_ => String::from("nil"),
			};

			let twin = match edge.twin {
				Some(ptr) => format!("e{}", ptr),
				_ => String::from("nil"),
			};

			/*
			let next = match edge.next {
				Some(next_ptr) => match self.get_edge_tuple(next_ptr) {
					Some((v1, v2)) => format!("e{},{}", v1, v2),
					_ => String::from("nil"),},
				_ => String::from("nil"),};
			*/

			/*
			let prev = match edge.prev {
				Some(prev_ptr) => match self.get_edge_tuple(prev_ptr) {
					Some((v1, v2)) => format!("e{},{}", v1, v2),
					_ => String::from("nil"),},
				_ => String::from("nil"),};
			*/

			/*
			let twin = match edge.twin {
				Some(twin_ptr) => match self.get_edge_tuple(twin_ptr) {
					Some((v1, v2)) => format!("e{},{}", v1, v2),
					_ => String::from("nil"),},
				_ => String::from("nil"),};
			*/

			let face = match edge.incident_face {
				Some(face_ptr) => format!("f{}", face_ptr),
				_ => String::from("nil"),};

			//Format: Edge Origin Twin Face Next Prev
			s.push_str(format!("\te{} {} {} {} {} {}\n", edge_str, origin, twin, next, prev, face).as_str());
		}
	
		s.push_str("\n\tFaces (index, inner component, outer component)\n");

		for face in &self.faces {
			let inner = {
				if !face.inner_component.is_empty() {
					format!("e{}", face.inner_component[0])
					/*
					match self.get_edge_tuple(face.inner_component[0]) {
						Some((v1, v2)) => format!("e{},{}", v1, v2),
						_ => String::from("nil 0"),}
					*/
				} else {
					String::from("nil")
				}
			};	
			
			let outer = {
				if !face.outer_component.is_empty() {
					format!("e{}", face.outer_component[0])
					/*
					match self.get_edge_tuple(face.outer_component[0]) {
						Some((v1, v2)) => format!("e{},{}", v1, v2),
						_ => String::from("nil"),}
					*/
				} else {
					String::from("nil")
				}
			};	

			s.push_str(format!("\tf{} {} {}\n", face.index(), inner, outer).as_str());
		}

		write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dcel_out() {
		let mut dcel = DoublyConnectedEdgeList::new();
		let v1 = dcel.new_vertex(&Point::new(6.0, 7.8));
		let v2 = dcel.new_vertex(&Point::new(-1.4, 3.6));
		let v3 = dcel.new_vertex(&Point::new(-5.8, -0.7));
		
		let e1 = dcel.new_edge(Some(v1));
		let e2 = dcel.new_edge(Some(v2));
		let e3 = dcel.new_edge(Some(v3));
		let e4 = dcel.new_edge(Some(v1));
		let e5 = dcel.new_edge(Some(v2));
		let e6 = dcel.new_edge(Some(v3));

		dcel.get_edge(e1).next = Some(e2);
		dcel.get_edge(e2).next = Some(e3);
		dcel.get_edge(e3).next = Some(e1);

		println!("{:?}", dcel);	
	}
}
