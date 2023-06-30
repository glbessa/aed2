use std::collections::LinkedList;
mod priority_queue;//::{PriorityQueue};
use priority_queue::priority_queue::PriorityQueue;
use std::convert::TryInto;

// -------------------
// Struct implementation

struct Graph {
    vertices: Vec<String>,
    relations: Vec<Vec<i32>>
}

impl Graph {
    fn new() -> Self {
        return Graph {
            vertices: Vec::new(),
            relations: Vec::new()
        };
    }

    fn insert_vertex(&mut self, vertex: &str) {
        self.vertices.push(vertex.to_string());

        self.relations.push(vec![0; self.vertices.len()]);

        for i in 0..self.relations.len() {
            self.relations[i].push(0);
        }
    }

    fn remove_vertex(&mut self, vertex_index: usize) -> Result<(), &'static str> {
        if self.vertices.len() <= vertex_index {
            return Err("Index out of range!");
        }

        self.vertices.remove(vertex_index);

        Ok(())
    }

    fn get_vertex(&self, vertex_idx: usize) -> Result<&String, &'static str> {
        if self.vertices.len() <= vertex_idx {
            return Err("Index out of range!");
        }

        Ok(&self.vertices[vertex_idx])
    }

    fn insert_edge(&mut self, src_idx: usize, dst_idx: usize, edge_weight: i32, directed: bool) -> Result<(), &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        self.relations[src_idx][dst_idx] = edge_weight;
        
        if directed == true {
            self.relations[dst_idx][src_idx] = edge_weight;
        }

        Ok(())
    }

    fn remove_edge(&mut self, src_idx: usize, dst_idx: usize, directed: bool) -> Result<(), &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        self.relations[src_idx][dst_idx] = 0;
        
        if directed == true {
            self.relations[dst_idx][src_idx] = 0;
        }        

        Ok(())
    }

    fn get_edge_weight(&self, src_idx: usize, dst_idx: usize) -> Result<i32, &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        let weight: i32 = self.relations[src_idx][dst_idx];

        Ok(weight)
    }

    fn get_adjacent_vertices(&self, vertex_idx: usize) -> Result<LinkedList<usize>, &'static str> {
        if self.vertices.len() <= vertex_idx {
            return Err("Index out of range!");
        }

        let mut adjacent_vertices: LinkedList<usize> = LinkedList::new();
        
        for i in 0..self.vertices.len() {
            if self.relations[vertex_idx][i] > 0 {
                adjacent_vertices.push_back(i);
            }
        }

        Ok(adjacent_vertices)
    }

    fn get_dijkstra_path(&self, src_idx: usize, dst_idx: usize) -> Result<LinkedList<usize>, &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        let mut previous_vertex: Vec<Option<usize>> = vec![None; self.vertices.len()];

        let mut path_cost: Vec<Option<usize>> = vec![None; self.vertices.len()];
        path_cost[src_idx] = Some(0);

        let mut is_closed: Vec<bool> = vec![false; self.vertices.len()];

        //let mut vertices_to_visit: Vec<usize> = Vec::new();
        let mut vertices_to_visit: LinkedList<usize> = LinkedList::new();
        let mut vert_to_visit: PriorityQueue<usize> = PriorityQueue::new();
        vertices_to_visit.push_back(src_idx);
        vert_to_visit.put(0, src_idx);

        let mut vertex_idx: usize;

        loop {
            /*
            vertex_idx = match vertices_to_visit.pop_back() {
                Some(v) => v,
                None => break
            };
            */

            vertex_idx = match vert_to_visit.pop() {
                Some((_, v)) => v,
                None => break
            };

            if is_closed[vertex_idx] {
                continue;
            }

            is_closed[vertex_idx] = true;

            match self.get_adjacent_vertices(vertex_idx) {
                Ok(list) => {
                    for idx in list.iter() {
                        if is_closed[*idx] {
                            continue;
                        }

                        if self.get_edge_weight(vertex_idx, *idx).unwrap() < 0 {
                            continue;
                        }

                        let total_cost: usize = match path_cost[vertex_idx] {
                            Some(cost_v_idx) => cost_v_idx + self.get_edge_weight(vertex_idx, *idx).unwrap() as usize,
                            None => self.get_edge_weight(vertex_idx, *idx).unwrap() as usize
                        };

                        match path_cost[*idx] {
                            Some(cost) => {
                                if cost > total_cost {
                                    path_cost[*idx] = Some(total_cost);
                                    previous_vertex[*idx] = Some(vertex_idx);
                                    //vertices_to_visit.push_front(*idx);
                                    vert_to_visit.put(total_cost.try_into().unwrap(), *idx);
                                }
                            },
                            None => {
                                path_cost[*idx] = Some(total_cost);
                                previous_vertex[*idx] = Some(vertex_idx);
                                //vertices_to_visit.push_front(*idx);
                                vert_to_visit.put(total_cost.try_into().unwrap(), *idx);
                            }
                        }
                    }
                },
                Err(msg) => return Err(msg)
            }
        }

        let mut path: LinkedList<usize> = LinkedList::new();
        path.push_front(dst_idx);
        let mut actual_vertex: usize = dst_idx;

        loop {
            actual_vertex = match previous_vertex[actual_vertex] {
                Some(v_idx) => v_idx,
                None => break
            };

            path.push_front(actual_vertex);
        }

        Ok(path)
    }
}
// -----------

fn main() {
    let mut graph: Graph = Graph::new();

    graph.insert_vertex("A");
    graph.insert_vertex("B");
    graph.insert_vertex("C");
    graph.insert_vertex("D");
    graph.insert_vertex("E");
    graph.insert_vertex("F");
    graph.insert_edge(0, 1, 2, false);
    graph.insert_edge(0, 2, 1, false);
    graph.insert_edge(1, 3, 1, false);
    graph.insert_edge(2, 3, 3, false);
    graph.insert_edge(2, 4, 4, false);
    graph.insert_edge(4, 5, 2, false);
    graph.insert_edge(3, 5, 2, false);
    let mut path = graph.get_dijkstra_path(0, 5).unwrap();

    for vertex in path.iter() {
        print!("{} ", graph.get_vertex(*vertex).unwrap());
    }

    println!("");

    let mut g2 = Graph::new();
    g2.insert_vertex("A");
    g2.insert_vertex("B");
    g2.insert_vertex("C");
    g2.insert_vertex("D");
    g2.insert_vertex("E");
    g2.insert_vertex("F");
    g2.insert_edge(0, 1, 10, false);
    g2.insert_edge(0, 3, 5, false);
    g2.insert_edge(3, 1, 3, false);
    g2.insert_edge(1, 2, 1, false);
    g2.insert_edge(3, 2, 8, false);
    g2.insert_edge(3, 4, 2, false);
    g2.insert_edge(4, 5, 6, false);
    g2.insert_edge(2, 4, 4, false);
    g2.insert_edge(2, 5, 4, false);
    path = g2.get_dijkstra_path(0, 5).unwrap();

    for vertex in path.iter() {
        print!("{} ", g2.get_vertex(*vertex).unwrap());
    }

    println!("");
}