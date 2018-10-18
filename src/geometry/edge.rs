
pub use geometry::face::Face;
pub use geometry::vertex::Vertex;

pub type VertexPtr = u32;
pub type EdgePtr = u32;
pub type FacePtr = u32;

#[derive(Copy, Clone, Eq, Debug)]
pub struct Edge {
	index : EdgePtr,
	pub is_inf : bool,
	pub origin : Option<VertexPtr>,
	pub next : Option<EdgePtr>,
	pub prev : Option<EdgePtr>,
	pub twin : Option<EdgePtr>,
	pub incident_face : Option<FacePtr>,
}

impl Edge {
	pub fn new(index : EdgePtr, from : Option<VertexPtr>) -> Edge {
		Edge {
			index : index,
			is_inf : false,
			origin : from,
			next : None,
			prev : None,
			twin : None,
			incident_face : None,
		}
	}
	pub fn index(&self) -> EdgePtr {
		self.index
	}
}

impl PartialEq for Edge {
	fn eq(&self, other : &Edge) -> bool {
		self.index == other.index
	}
}