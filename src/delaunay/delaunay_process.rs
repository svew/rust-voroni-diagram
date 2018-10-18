
use std::collections::HashMap;

use geometry::dcel::*;

struct DelaunayProcess {
    dcel_in : DoublyConnectedEdgeList,
    dcel_out : DoublyConnectedEdgeList,
    vertex_map : HashMap<u32, u32>, //FacePtr to VertexPtr
}

impl DelaunayProcess {
    pub fn DelaunayProcess(dcel : DoublyConnectedEdgeList) -> DelaunayProcess {
        DelaunayProcess {
            dcel_in : dcel
            dcel_out : DoublyConnectedEdgeList::new();
            vertex_map : HashMap::new(),
        }
    }

    pub fn execute(&mut self) {
        self.add_vertices();
    }

    //Add the vertices of what will be the completed Delaunay Triangulation
    fn add_vertices(&mut self) {
        for (index, face) in &self.dcel_in.faces {
            if let Some(site_ptr) = face.site {
                let point = *self.dcel_in.get_vertex(site_ptr).point;
                let out_index = self.dcel_out.new_vertex(point);
                self.vertex_map.insert(*index, out_index);
            }
        }
    }

    fn add_edges(&mut self) {
        let key_list = Vec::new();

        while !self.dcel_in.edges.is_empty() {
            //Pop out an twin half edges, read the two adjacent faces to find the sites, and connect the
            //sites with a dual edge
        }
    }
}