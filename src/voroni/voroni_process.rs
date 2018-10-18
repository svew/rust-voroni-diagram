
#![allow(dead_code)]

use std::collections::BinaryHeap;

use geometry::dcel::DoublyConnectedEdgeList;
use geometry::point::Point;
use voroni::event::*;
use voroni::geometry;
use voroni::status_struct::*;

/////////////////////////////////////////////////////////////////

pub enum ParabolaResult {
	None, //There are no parabolas in the diagram
	ToLeftOf(u32), //Site is not below an arc, but to the left of one (colinear)
	ToRightOf(u32), //Site is not below an arc, but to the right of one (colinear)
	Intersecting(u32), //Site intersects a single arc
}

/////////////////////////////////////////////////////////////////

pub struct VoroniProcess {
	event_queue_index : u32,
	dcel : DoublyConnectedEdgeList,
	event_queue : BinaryHeap<Event>,
	status_struct : StatusStruct,
	iteration : u32,
}

impl VoroniProcess {
	pub fn new(sites : Vec<(i32, i32)>) -> VoroniProcess {
		let mut vp = VoroniProcess {
			event_queue_index : 0,
			dcel : DoublyConnectedEdgeList::new(),
			event_queue : BinaryHeap::new(),
			status_struct : StatusStruct::new(),
			iteration : 0,
		};
		for site in sites {
			vp.push_site_event(&Point::new(site.0 as f64, site.1 as f64));
		}
		return vp;
	}

	pub fn get_dcel(&self) -> &DoublyConnectedEdgeList {
		&self.dcel
	}

	pub fn step(&mut self) -> bool {
		let event_option = self.event_queue.pop();

		match event_option {
			Some(event) => match event {
				Event::Site(site) => {
					println!("Iteration {}: Site event at {:?}", self.iteration, site);
					self.process_site(site);
				},
				Event::Circle(center, radius, status_pointer, event_pointer, is_valid) => {
					if is_valid.value {
						println!("Iteration {}: Circle event at {:?}, radius of {}", self.iteration, center, radius);
						self.process_circle(center, status_pointer, event_pointer);
					}
				},
			},
			None => return false,
		}
		println!("\n{:?}",self.status_struct);
		self.iteration += 1;

		return true;
	}

	pub fn execute(&mut self) {
		while self.step() {}
	}

	pub fn push_circle_event(&mut self, center : &Point, radius : f64, status_pointer : u32) {
		let new_event = Event::Circle(*center, radius, status_pointer, self.event_queue_index, IsValid::new());
		self.event_queue_index += 1;
		self.event_queue.push(new_event);
	}

	pub fn push_site_event(&mut self, site : &Point) {
		self.event_queue.push(Event::Site(*site));
	}

	fn process_site(&mut self, new_site : Point) {
		/*
		3. Replace the leaf of 'T' that represents 'a' with a subtree having 3
		leaves. The middle leaf stores the new site p_i and the other two leaves
		store the site p_j that was originally stored with 'a'. Store tuples
		(p_j,p_i) and (p_i,p_j) representing the new breakpoints at the two new
		internal nodes. Perform rebalancing operations on 'T' if necessary
		*/

		/*
		4. Create a new half-edge records in the Voroni diagram structure for the
		edge seperating V(p_i) and V(p_j), which will be traced out by the two
		new breakpoints.
		*/

		let new_vertex = self.dcel.new_vertex(&new_site);
		self.dcel.get_vertex(new_vertex).is_site = true;
		let new_face = self.dcel.new_face(Some(new_vertex));
		let parabola_result = self.get_parabola_by_x(&new_site, new_site.y());

		let (new_root_ptr, new_site_ptr, replace_ptr_option) = match parabola_result {
			ParabolaResult::ToLeftOf(leaf_ptr) => {
				println!("\tThe site event intersected parabola {} once on the left", leaf_ptr);
				/*        lr
						/    \
					   l	  r		
				*/

				let (old_site, old_face) = match self.status_struct.get(leaf_ptr).node_type {
					NodeType::Internal(break_point) => panic!("We should not be intersecting an internal node!"),
					NodeType::Leaf(arc) => (arc.site, arc.face_ptr),};

				let (new_edge_o, new_edge_n) = self.dcel.new_dual_edges(None, None);
				self.dcel.get_face(new_face).outer_component.push(new_edge_n);
				self.dcel.get_face(old_face).outer_component.push(new_edge_o);
				self.dcel.get_edge(new_edge_n).incident_face = Some(new_face);
				self.dcel.get_edge(new_edge_o).incident_face = Some(old_face);

				let left_ptr = self.status_struct.new_leaf(&new_site, new_face);
				let right_ptr = self.status_struct.new_leaf(&old_site, old_face);
				let lr_ptr = self.status_struct.new_internal(
					BreakPoint {
						half_edge : new_edge_n,
						left_site : new_site,
						right_site : old_site},
					left_ptr,
					right_ptr);

				self.status_struct.get_mut(left_ptr).parent = Some(lr_ptr);
				self.status_struct.get_mut(right_ptr).parent = Some(lr_ptr);

				(lr_ptr, left_ptr, Some(leaf_ptr))
			},
			ParabolaResult::ToRightOf(leaf_ptr) => {
				println!("\tThe site event intersected parabola {} once on the right", leaf_ptr);
				/*        lr
						/    \
					   l	  r		
				*/

				let (old_site, old_face) = match self.status_struct.get(leaf_ptr).node_type {
					NodeType::Internal(break_point) => panic!("We should not be intersecting an internal node!"),
					NodeType::Leaf(arc) => (arc.site, arc.face_ptr),};

				let (new_edge_o, new_edge_n) = self.dcel.new_dual_edges(None, None);
				self.dcel.get_face(new_face).outer_component.push(new_edge_n);
				self.dcel.get_face(old_face).outer_component.push(new_edge_o);
				self.dcel.get_edge(new_edge_n).incident_face = Some(new_face);
				self.dcel.get_edge(new_edge_o).incident_face = Some(old_face);

				let left_ptr = self.status_struct.new_leaf(&old_site, old_face);
				let right_ptr = self.status_struct.new_leaf(&new_site, new_face);
				let lr_ptr = self.status_struct.new_internal(
					BreakPoint {
						half_edge : new_edge_o,
						left_site : old_site,
						right_site : new_site},
					left_ptr,
					right_ptr);

				self.status_struct.get_mut(left_ptr).parent = Some(lr_ptr);
				self.status_struct.get_mut(right_ptr).parent = Some(lr_ptr);

				(lr_ptr, right_ptr, Some(leaf_ptr))
			},
			ParabolaResult::Intersecting(leaf_ptr) => {
				println!("\tThe site event intersected parabola {} in the middle", leaf_ptr);
				/*
				          ir
						/    \
					   lm	  r
					  /  \
				     l    m 		
				*/
				let (old_site, old_face) = match self.status_struct.get(leaf_ptr).node_type {
					NodeType::Internal(break_point) => panic!("We should not be intersecting an internal node!"),
					NodeType::Leaf(arc) => (arc.site, arc.face_ptr),};
				let (new_edge_n, new_edge_o) = self.dcel.new_dual_edges(None, None);
				self.dcel.get_face(new_face).outer_component.push(new_edge_n);
				self.dcel.get_face(old_face).outer_component.push(new_edge_o);
				self.dcel.get_edge(new_edge_n).incident_face = Some(new_face);
				self.dcel.get_edge(new_edge_o).incident_face = Some(old_face);
				let left_ptr = self.status_struct.new_leaf(&old_site, old_face);
				let middle_ptr = self.status_struct.new_leaf(&new_site, new_face);
				let right_ptr = self.status_struct.new_leaf(&old_site, old_face);
				let lm_ptr = self.status_struct.new_internal(
					BreakPoint {
						half_edge : new_edge_o,
						left_site : old_site,
						right_site : new_site},
					left_ptr,
					middle_ptr);
				let ir_ptr = self.status_struct.new_internal(
					BreakPoint {
						half_edge : new_edge_n,
						left_site : new_site,
						right_site : old_site},
					lm_ptr,
					right_ptr);

				self.status_struct.get_mut(left_ptr).parent = Some(lm_ptr);
				self.status_struct.get_mut(middle_ptr).parent = Some(lm_ptr);
				self.status_struct.get_mut(right_ptr).parent = Some(ir_ptr);
				self.status_struct.get_mut(lm_ptr).parent = Some(ir_ptr);
				(ir_ptr, middle_ptr, Some(leaf_ptr))
			},
			ParabolaResult::None => {
				println!("\tThe site event didn't intersect anything");
				let new_leaf_ptr = self.status_struct.new_leaf(&new_site, new_face);
				
				(new_leaf_ptr, new_leaf_ptr, None)
			},
		};

		//Time to replace the old arc with our new structure
		match replace_ptr_option {
			Some(replace_ptr) => match self.status_struct.get(replace_ptr).parent {
					//The node we're replacing has a parent
					Some(parent_ptr) => {
						if self.status_struct.get(parent_ptr).left.unwrap() == replace_ptr {
							self.status_struct.get_mut(parent_ptr).left = Some(new_root_ptr);
						} else {
							self.status_struct.get_mut(parent_ptr).right = Some(new_root_ptr);
						}
						self.status_struct.get_mut(new_root_ptr).parent = Some(parent_ptr);
					},
					//The node we're replacing is root
					None => self.status_struct.head = Some(new_root_ptr),
				},
			None => self.status_struct.head = Some(new_root_ptr),
		}
		/*
		5. Check the triple of consecutive arcs where the new arc for p_i is the
		left arc to see if the breakpoints converge. If so, insert the circle
		into 'Q' and add pointers between the node in 'T' and the node in 'Q'.
		Do the same for the triple where the new arc is the right arc.
		*/
		if let Some(left_triple) = self.status_struct.get_left_triple(new_site_ptr) {
			println!("\tChecking leftward triple {:?} {:?} {:?}", left_triple.0, left_triple.1, left_triple.2);
			if geometry::is_clockwise(&left_triple)  {
				println!("\tThe leftward triple converges!");
				let left_arc = self.status_struct.get_left_arc(Some(new_site_ptr)).unwrap();
				self.make_circle_event(left_arc, &left_triple);
			}
		}
		if let Some(right_triple) = self.status_struct.get_right_triple(new_site_ptr) {
			println!("\tChecking rightward triple {:?} {:?} {:?}", right_triple.0, right_triple.1, right_triple.2);
			if geometry::is_clockwise(&right_triple)  {
				println!("\tThe rightward triple converges!");
				let right_arc = self.status_struct.get_right_arc(Some(new_site_ptr)).unwrap();
				self.make_circle_event(right_arc, &right_triple);
			}
		}
	}

	fn remove_circle_event(&mut self, leaf_ptr : u32) {
		let mut circle_event = None;
		if let NodeType::Leaf(ref mut arc) = self.status_struct.get_mut(leaf_ptr).node_type {
			circle_event = arc.event_ptr;
			arc.event_ptr = None;
		}
		if let Some(circle_event_ptr) = circle_event {
			for event in &self.event_queue {
				match event {
					&Event::Circle(_,_,_, event_id, mut is_valid) => if event_id == circle_event_ptr {
						is_valid.value = false;
						break;
					}
					_ => (),
				}
			}
		}
	}

	fn process_circle(&mut self, center : Point, leaf_middle_ptr : u32, event_pointer : u32) {
		
		/*
		1. Delete the leaf γ that represents the disappearing arc α from T. 
		Update the tuples representing the breakpoints at the internal nodes. 
		Perform	rebalancing operations on T if necessary. Delete all circle 
		events involving α from Q; these can be found using the pointers from 
		the predecessor and the successor of γ in T. (The circle event where α 
		is the middle arc is currently being handled, and has already been 
		deleted from Q.)
		*/

		let leaf_left_ptr = self.status_struct.get_left_arc(Some(leaf_middle_ptr)).unwrap();
		let leaf_right_ptr = self.status_struct.get_right_arc(Some(leaf_middle_ptr)).unwrap();

		let (pred_ptr, succ_ptr, parent_ptr, other_ptr) = {
			let ref mut ss = &mut self.status_struct;

			let pred_ptr = ss.predecessor(leaf_middle_ptr).unwrap();
			let succ_ptr = ss.successor(leaf_middle_ptr).unwrap();
			let parent_ptr = ss.get(leaf_middle_ptr).parent.unwrap();
			let grandparent_ptr = ss.get(parent_ptr).parent.unwrap();

			let other_ptr = if parent_ptr == pred_ptr { succ_ptr } else { pred_ptr };

			let sibling_ptr;
			if ss.get(parent_ptr).right.unwrap() == leaf_middle_ptr {
				sibling_ptr = ss.get(parent_ptr).left.unwrap();
			} else if ss.get(parent_ptr).left.unwrap() == leaf_middle_ptr {
				sibling_ptr = ss.get(parent_ptr).right.unwrap();
			} else {
				panic!("Graph error: Disconnect in finding sibling");
			}

			ss.get_mut(sibling_ptr).parent = Some(grandparent_ptr);
			if ss.get(grandparent_ptr).left.unwrap() == parent_ptr {
				ss.get_mut(grandparent_ptr).left = Some(sibling_ptr);
			} else if ss.get(grandparent_ptr).right.unwrap() == parent_ptr {
				ss.get_mut(grandparent_ptr).right = Some(sibling_ptr);
			} else {
				panic!("Graph error: Disconnect in setting grandparent child");
			}
			/*
			if let NodeType::Internal(ref mut break_point) = ss.get_mut(other_ptr).node_type {
				break_point.left_site = match ss.get(ss.predecessor(other_ptr).unwrap()).node_type {
					NodeType::Leaf(arc) => arc.site,
					_ => panic!("Fuck"),
				};
				break_point.right_site = match ss.get(ss.successor(other_ptr).unwrap()).node_type {
					NodeType::Leaf(arc) => arc.site,
					_ => panic!("Fuck"),
				};
			}
			*/
			if other_ptr == pred_ptr {
				let new_other_succ = ss.successor(other_ptr).unwrap();
				let new_site = ss.get_site(Some(new_other_succ)).unwrap();
				ss.set_right_site(other_ptr, new_site);
			} else {
				let new_other_pred = ss.predecessor(other_ptr).unwrap();
				let new_site = ss.get_site(Some(new_other_pred)).unwrap();
				ss.set_left_site(other_ptr, new_site);
			}

			(pred_ptr, succ_ptr, parent_ptr, other_ptr)
		};

		self.remove_circle_event(leaf_left_ptr);
		self.remove_circle_event(leaf_right_ptr);

		/*
		2. Add the center of the circle causing the event as a vertex record to 
		the doubly-connected edge list D storing the Voronoi diagram under 
		construction. Create two half-edge records corresponding to the new 
		breakpoint of the beach line. Set the pointers between them 
		appropriately. Attach the three new records to the half-edge records that
		end at the vertex.
		*/

		let (twin1, twin2) = self.dcel.new_dual_edges(None, None);
		if let NodeType::Leaf(arc) = self.status_struct.get(leaf_left_ptr).node_type {
			self.dcel.get_edge(twin1).incident_face = Some(arc.face_ptr);
		}
		if let NodeType::Leaf(arc) = self.status_struct.get(leaf_right_ptr).node_type {
			self.dcel.get_edge(twin2).incident_face = Some(arc.face_ptr);
		}

		let center_vertex_ptr = self.dcel.new_vertex(&center);
		self.dcel.get_vertex(center_vertex_ptr).incident_edge = Some(twin1);


		// hook up next pointers on halfedges
		let pred_edge = self.status_struct.get_edge(pred_ptr);
		let succ_edge = self.status_struct.get_edge(succ_ptr);
		let parent_edge = self.status_struct.get_edge(parent_ptr);
		let other_edge = self.status_struct.get_edge(other_ptr);

		let pred_edge_twin = self.dcel.get_edge(pred_edge).twin.unwrap();
		let succ_edge_twin = self.dcel.get_edge(succ_edge).twin.unwrap();

		self.dcel.get_edge(parent_edge).origin = Some(center_vertex_ptr);
		self.dcel.get_edge(other_edge).origin = Some(center_vertex_ptr);
		self.dcel.get_edge(twin1).origin = Some(center_vertex_ptr);

		self.dcel.get_edge(pred_edge_twin).next = Some(succ_edge);
		self.dcel.get_edge(succ_edge_twin).next = Some(twin1);
		self.dcel.get_edge(twin2).next = Some(pred_edge);

		/*
		3. Check the new triple of consecutive arcs that has the former left 
		neighbor of α as its middle arc to see if the two breakpoints of the 
		triple converge. If so, insert the corresponding circle event into Q. and
		set pointers between the new circle event in Q and the corresponding leaf
		of T. Do the same for the triple where the former right neighbor is the 
		middle arc.
		*/

		if let NodeType::Internal(ref mut break_point) = self.status_struct.get_mut(other_ptr).node_type {
			break_point.half_edge = twin2;
		}

		if let Some(left_triple) = self.status_struct.get_middle_triple(leaf_left_ptr) {
			println!("\tChecking leftward triple {:?}, {:?}, {:?}", left_triple.0, left_triple.1, left_triple.2);
			if geometry::is_clockwise(&left_triple) {
				println!("\tFound converging triple");
				self.make_circle_event(leaf_left_ptr, &left_triple);
			}
		}
		if let Some(right_triple) = self.status_struct.get_middle_triple(leaf_right_ptr) {
			println!("\tChecking rightward triple {:?}, {:?}, {:?}", right_triple.0, right_triple.1, right_triple.2);
			if geometry::is_clockwise(&right_triple) {
				println!("\tFound converging triple");
				self.make_circle_event(leaf_right_ptr, &right_triple);
			}
		}
	}

	fn get_parabola_by_x(&mut self, site : &Point, line_y : f64) -> ParabolaResult {

		let mut ss = &mut self.status_struct;
	
		//If there is no head node, there is nothing we can point to
		let mut iter_ptr = match ss.head {
			Some(node_ptr) => node_ptr,
			None => return ParabolaResult::None,};

		let mut last_direction = "none";

		loop {
			let iter_node = ss.map.get(&iter_ptr).unwrap();	
			match iter_node.node_type {

				//An intersection between two parabolas, must compare to see which side this site lands on
				NodeType::Internal(break_point) => {
					//Find the x position where the two parabolas cross for this node
					let intersection_x = geometry::get_parabola_intersection_x (
							&break_point.left_site, 
							&break_point.right_site, 
							line_y);

					if site.x() > intersection_x {
						iter_ptr = iter_node.right.unwrap();
						last_direction = "right";
					} else {
						iter_ptr = iter_node.left.unwrap();
						last_direction = "left";
					}
				},

				//A parabola, 
				NodeType::Leaf(arc) => {
					//If the two have the same y coordinate, then they're horizontally colinear
					if arc.site.y() == site.y() {
						match last_direction {
							"left" => return ParabolaResult::ToLeftOf(iter_ptr),
							"right" => return ParabolaResult::ToRightOf(iter_ptr),
							_ => {
								if arc.site.x() < site.x() {
									return ParabolaResult::ToRightOf(iter_ptr);
								} else {
									return ParabolaResult::ToLeftOf(iter_ptr);
								}
							},
						}
					//Otherwise, everything is hunky dory! :)
					} else {
						return ParabolaResult::Intersecting(iter_ptr);
					}
				},
			}
		}
	}

	fn make_circle_event(&mut self, leaf_ptr : u32, triple : &(Point, Point, Point)) {
		if let Some(center) = geometry::get_circle_center(triple) {
			println!("\tMaking a new circle event at {:?}", center);
			let radius = geometry::get_distance(&center, &triple.0);
			self.push_circle_event(&center, radius, leaf_ptr);
		}
	}
}
