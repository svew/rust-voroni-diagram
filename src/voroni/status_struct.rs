
#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

use geometry::point::Point;

/////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct StatusStruct {
    node_index : u32,
    pub map : HashMap<u32, StatusNode>,
    pub head : Option<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatusNode {
    index : u32,
    pub node_type : NodeType,
	pub left : Option<u32>,
	pub right : Option<u32>,
    pub parent : Option<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NodeType {
    Internal(BreakPoint), //Represent intersections between parabolas on the beach line
    Leaf(Arc), //Represents an arc, about the given site, and an event pointer
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arc {
	pub site : Point,
	pub face_ptr : u32,
	pub event_ptr : Option<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreakPoint {
	pub left_site : Point,
	pub right_site : Point,
	pub half_edge : u32,
}

/////////////////////////////////////////////////////////////////

impl StatusStruct {
    pub fn new() -> StatusStruct {
        StatusStruct {
			node_index : 0,
			map : HashMap::new(),
			head : None,
		}
    }
	
	pub fn is_empty(&self) -> bool {
		self.head.is_some()
	}
	
	pub fn new_leaf(&mut self, site : &Point, face_ptr : u32) -> u32 {
		let new_index = self.node_index;
		let leaf = StatusNode::new_leaf(new_index, site, face_ptr);
		self.map.insert(new_index, leaf);
		self.node_index += 1;
		return new_index;
	}
	
	pub fn new_internal(&mut self, break_point : BreakPoint, left : u32, right : u32) -> u32 {
		let new_index = self.node_index;
		let internal = StatusNode::new_internal(new_index, break_point, left, right);
		self.map.insert(new_index, internal);
		self.node_index += 1;
		return new_index;
	}
	
	pub fn tree_max(&self, root_ptr : u32) -> u32 {
		let mut iter_node = root_ptr;
		while let Some(right) = self.get(iter_node).right {
			iter_node = right;
		}
		return iter_node;
	}
	
	pub fn tree_min(&self, root_ptr : u32) -> u32 {
		let mut iter_node = root_ptr;
		while let Some(left) = self.get(iter_node).left {
			iter_node = left;
		}
		return iter_node;
	}
	
	pub fn successor(&self, node_ptr : u32) -> Option<u32> {
		//If this node has a right node, then the min of that subtree is successor
		if let Some(right) = self.get(node_ptr).right {
			return Some(self.tree_min(right));
		}
		
		let mut iter_node = Some(node_ptr);
		let mut iter_parent = self.get(node_ptr).parent;
		//While the iter has a parent, and that parent's right node is iter
		while iter_parent.is_some() && iter_node == self.get(iter_parent.unwrap()).right {
			//Go up the tree
			iter_node = iter_parent;
			iter_parent = self.get(iter_parent.unwrap()).parent;
		}
		return iter_parent;
	}
	
	pub fn predecessor(&self, node_ptr : u32) -> Option<u32> {
		if let Some(left) = self.get(node_ptr).left {
			return Some(self.tree_max(left));
		}
		
		let mut iter_node = Some(node_ptr);
		let mut iter_parent = self.get(node_ptr).parent;
		while iter_parent.is_some() && iter_node == self.get(iter_parent.unwrap()).left {
			iter_node = iter_parent;
			iter_parent = self.get(iter_parent.unwrap()).parent;
		}
		return iter_parent;
	}
	
	pub fn get_left_arc(&self, node_ptr : Option<u32>) -> Option<u32> {
		return node_ptr
			.and_then(|node| self.predecessor(node))
			.and_then(|left| self.predecessor(left));
	}
	
	pub fn get_right_arc(&self, node_ptr : Option<u32>) -> Option<u32> {
		return node_ptr
			.and_then(|node| self.successor(node))
			.and_then(|right| self.successor(right));
	}
	
	pub fn get_left_triple(&self, node_ptr : u32) -> Option<(Point, Point, Point)> {
        let left_arc = self.get_left_arc(Some(node_ptr));
        let left_left_arc = self.get_left_arc(left_arc);

        let this_site = self.get_site(Some(node_ptr));
        let left_site = self.get_site(left_arc);
        let left_left_site = self.get_site(left_left_arc);

        if this_site.is_some() && left_site.is_some() && left_left_site.is_some() {
            return Some((left_left_site.unwrap(), left_site.unwrap(), this_site.unwrap()));
        } else { 
			return None;
		}
	}
	
	pub fn get_middle_triple(&self, node_ptr : u32) -> Option<(Point, Point, Point)> {
        let right_arc = self.get_right_arc(Some(node_ptr));
        let left_arc = self.get_left_arc(Some(node_ptr));

        let this_site = self.get_site(Some(node_ptr));
        let right_site = self.get_site(right_arc);
        let left_site = self.get_site(left_arc);

        if this_site.is_some() && right_site.is_some() && left_site.is_some() {
            return Some((left_site.unwrap(), this_site.unwrap(), right_site.unwrap()));
        } else { 
			return None;
		}
	}
	
	pub fn get_right_triple(&self, node_ptr : u32) -> Option<(Point, Point, Point)> {
        let right_arc = self.get_right_arc(Some(node_ptr));
        let right_right_arc = self.get_right_arc(right_arc);

        let this_site = self.get_site(Some(node_ptr));
        let right_site = self.get_site(right_arc);
        let right_right_site = self.get_site(right_right_arc);

        if this_site.is_some() && right_site.is_some() && right_right_site.is_some() {
            return Some((this_site.unwrap(), right_site.unwrap(), right_right_site.unwrap()));
        } else { 
			return None;
		}
	}
	
	pub fn set_right_site(&mut self, node_ptr : u32, site : Point) {
        if let NodeType::Internal(ref mut bp) = self.get_mut(node_ptr).node_type {
            bp.right_site = site;
        } else {
            panic!("target of set_site should be internal");
        }
    }

    pub fn set_left_site(&mut self, node_ptr : u32, site : Point) {
        if let NodeType::Internal(ref mut bp) = self.get_mut(node_ptr).node_type {
            bp.left_site = site;
        } else {
            panic!("target of set_site should be internal");
        }
    }

	pub fn get_site(&self, node_ptr_option : Option<u32>) -> Option<Point> {
		match node_ptr_option {
			Some(node_ptr) => match self.get(node_ptr).node_type {
				NodeType::Leaf(arc) => Some(arc.site),
				NodeType::Internal(_) => None,
			},
			None => None,
		}
	}
	
	pub fn get_mut(&mut self, index : u32) -> &mut StatusNode {
		return self.map.get_mut(&index).unwrap();
	}
	
	pub fn get(&self, index : u32) -> &StatusNode {
		return self.map.get(&index).unwrap();
	}

	fn iter_fmt(&self, string : &mut String, root_ptr : u32) {
		let root_option = self.map.get(&root_ptr);
		match root_option {
			Some(root) => {
				if let Some(left_ptr) = root.left {
					self.iter_fmt(string, left_ptr);
				}
				let msg = format!("{:?}\n", root);
				string.push_str(msg.as_str());
				if let Some(right_ptr) = root.right {
					self.iter_fmt(string, right_ptr);
				}
			},
			None => {
				let msg = format!("\t\tNode {} was not found\n", root_ptr);
				string.push_str(msg.as_str());
			}
		}
	}

	pub fn get_edge(&self, index : u32) -> u32 {
		match self.get(index).node_type {
			NodeType::Internal(break_point) => break_point.half_edge,
			_ => panic!("Can't get a half edge from a leaf node!"),
		}
	}
}

impl fmt::Debug for StatusStruct {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		let mut msg = String::from("\t\tStatus Structure:\n");
		match self.head {
			Some(head_ptr) => self.iter_fmt(&mut msg, head_ptr),
			_ => (),
		}
		write!(f, "{}", msg.as_str())
	}
}

/////////////////////////////////////////////////////////////////

impl StatusNode {
	pub fn new_leaf(index : u32, site : &Point, face_ptr : u32) -> StatusNode {
		StatusNode {
			index : index,
			node_type : NodeType::Leaf(Arc::new(site, face_ptr)),
			left : None,
			right : None,
			parent : None,
		}
	}
	
	pub fn new_internal(index : u32, break_point : BreakPoint, left : u32, right : u32) -> StatusNode {
		StatusNode {
			index : index,
			node_type : NodeType::Internal(break_point),
			left : Some(left),
			right : Some(right),
			parent : None,
		}
	}
	
    pub fn index(&self) -> u32 {
        self.index
    }
}

impl fmt::Debug for StatusNode {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		match self.node_type {
			NodeType::Internal(break_point) => write!(f, "\t\tInternal({})\tl:{:?} r:{:?}   {:?},{:?}",
					self.index(), self.left, self.right, break_point.left_site, break_point.right_site),
			NodeType::Leaf(arc) => write!(f, "\t\tLeaf({})  \ts:{:?}", self.index(), arc.site),
		}
	}
}

/////////////////////////////////////////////////////////////////

impl Arc {
	pub fn new(site : &Point, face_ptr : u32) -> Arc {
		Arc {
			site : *site,
			face_ptr : face_ptr,
			event_ptr : None,
		}
	}
}

/////////////////////////////////////////////////////////////////

