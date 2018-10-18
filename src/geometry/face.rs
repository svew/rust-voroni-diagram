
use rand;
use rand::Rng;

pub type VertexPtr = u32;
pub type EdgePtr = u32;
pub type FacePtr = u32;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Face {
	index : FacePtr,
	color : [f32; 3],
	pub inner_component: Vec<EdgePtr>,
	pub outer_component: Vec<EdgePtr>,
	pub site : Option<VertexPtr>,
}

impl Face {
	pub fn new(index : FacePtr, site : Option<VertexPtr>) -> Face {
    	let mut rng = rand::thread_rng();
		Face {
			index : index,
			color : [rng.next_f32(), rng.next_f32(), rng.next_f32()],
			inner_component : Vec::new(),
			outer_component : Vec::new(),
			site : site,
		}
	}
	pub fn index(&self) -> FacePtr {
		self.index
	}
	pub fn color(&self) -> [f32; 3] {
		self.color
	}
}
