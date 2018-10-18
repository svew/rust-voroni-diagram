
use geometry::point::Point;

pub type VertexPtr = u32;
pub type EdgePtr = u32;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	index : VertexPtr,
	pub point : Point,
	pub incident_edge : Option<EdgePtr>,
	pub is_site : bool,
}

impl Vertex {
	pub fn new(index : VertexPtr, point : Point) -> Vertex {
		Vertex {
			index : index, 
			point : point, 
			incident_edge : None,
			is_site : false,
		}
	}
	pub fn index(&self) -> VertexPtr {
		self.index
	}
}

impl PartialEq for Vertex {
	fn eq(&self, other : &Vertex) -> bool {
		self.index == other.index
	}
}