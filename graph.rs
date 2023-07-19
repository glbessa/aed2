// https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
use std::collections::{LinkedList, HashSet, BinaryHeap};
use std::cmp::{Ordering, Reverse};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::convert::TryInto;
use std::fmt;

// Edge Struct Declaration -------------
pub struct Edge {
    src: u32,
    dst: u32,
    weight: i32
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Eq for Edge {
    fn assert_receiver_is_total_eq(&self) {

    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge (src: {}; dst: {}; weight: {})\n", self.src, self.dst, self.weight);
        Ok(())
    }
}
// -------------------------------

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<String>,
    relations: Vec<Vec<i32>>
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph (\n");
        for i in 0..self.relations.len() {
            for j in 0..self.relations.len() {
                if self.get_edge_weight(i, j).unwrap() != 0 {
                    write!(f, "\t{} -- ({}) --> {}\n", self.get_vertex(i).unwrap(), self.get_edge_weight(i, j).unwrap(), self.get_vertex(j).unwrap());
                }
            }
        }
        write!(f, ")\n");

        Ok(())
    }
}

impl Graph {
    pub fn new() -> Self {
        return Graph {
            vertices: Vec::new(),
            relations: Vec::new()
        };
    }

    pub fn from(vertices_ids: Vec<String>, adjacency_matrix: Vec<Vec<i32>>) -> Self {
        Graph {
            vertices: vertices_ids,
            relations: adjacency_matrix
        }
    }

    pub fn from_file(file_name: String) -> Result<Self, &'static str> {
        let mut f = File::open(file_name)
            .expect("Error while opening file!");

        let mut content: String = String::new();
        f.read_to_string(&mut content)
            .expect("Error while reading file!");

        let vertices_ids: Vec<String>;
        let mut adjacency_matrix: Vec<Vec<i32>> = Vec::new();
        let mut lines: Vec<&str> = content.split("\n").collect();

        vertices_ids = lines.remove(0).split(",").map(|s| String::from(s)).collect();

        // Faz algo muito louco
        /*
        let _ = lines.into_iter()
            .map(|line| line.split(" ")
                            .map(|s| s.parse::<i32>())
                            .map(Result::unwrap)
                            .collect())
            .map(|v| adjacency_matrix.push(v));
        */
        
        for line in lines.into_iter() {
            let t = line.split(" ")
                .map(|s| s.parse::<i32>())
                .map(Result::unwrap)
                .collect();

            adjacency_matrix.push(t);
        }

        Ok(Graph::from(vertices_ids, adjacency_matrix))        
    }

    pub fn insert_vertex(&mut self, vertex: &str) {
        self.vertices.push(vertex.to_string());

        self.relations.push(vec![0; self.vertices.len()]);

        for i in 0..self.relations.len() {
            self.relations[i].push(0);
        }
    }

    pub fn remove_vertex(&mut self, vertex_index: usize) -> Result<(), &'static str> {
        if self.vertices.len() <= vertex_index {
            return Err("Index out of range!");
        }

        self.vertices.remove(vertex_index);

        Ok(())
    }

    pub fn get_vertex(&self, vertex_idx: usize) -> Result<&String, &'static str> {
        if self.vertices.len() <= vertex_idx {
            return Err("Index out of range!");
        }

        Ok(&self.vertices[vertex_idx])
    }

    pub fn insert_edge(&mut self, src_idx: usize, dst_idx: usize, edge_weight: i32, directed: bool) -> Result<(), &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        self.relations[src_idx][dst_idx] = edge_weight;
        
        if directed == true {
            self.relations[dst_idx][src_idx] = edge_weight;
        }

        Ok(())
    }

    pub fn remove_edge(&mut self, src_idx: usize, dst_idx: usize, directed: bool) -> Result<(), &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        self.relations[src_idx][dst_idx] = 0;
        
        if directed == true {
            self.relations[dst_idx][src_idx] = 0;
        }        

        Ok(())
    }

    pub fn get_edge_weight(&self, src_idx: usize, dst_idx: usize) -> Result<i32, &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        let weight: i32 = self.relations[src_idx][dst_idx];

        Ok(weight)
    }

    pub fn get_adjacent_vertices(&self, vertex_idx: usize) -> Result<LinkedList<usize>, &'static str> {
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

    pub fn get_dijkstra_path(&self, src_idx: usize, dst_idx: usize) -> Result<LinkedList<usize>, &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        let mut previous_vertex: Vec<Option<usize>> = vec![None; self.vertices.len()];

        let mut path_cost: Vec<Option<usize>> = vec![None; self.vertices.len()];
        path_cost[src_idx] = Some(0);

        let mut is_closed: Vec<bool> = vec![false; self.vertices.len()];

        let mut vertices_to_visit: LinkedList<usize> = LinkedList::new();
        let mut vert_to_visit: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        vertices_to_visit.push_back(src_idx);
        vert_to_visit.push((0, src_idx));

        let mut vertex_idx: usize;

        loop {
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
                                    vert_to_visit.push((total_cost.try_into().unwrap(), *idx));
                                }
                            },
                            None => {
                                path_cost[*idx] = Some(total_cost);
                                previous_vertex[*idx] = Some(vertex_idx);
                                vert_to_visit.push((total_cost.try_into().unwrap(), *idx));
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

    // Algoritmo de Kruskal: o conjunto A é  uma floresta cujos vértices são todos os vértices do grafo e a aresta segura
    //   adicionada é sempre uma aresta de peso mínimo no grafo que conecta duas componentes distintas.
    pub fn get_mst_kruskal(&self) -> Self {
        let mut a: HashSet<(usize, usize)> = HashSet::new();
        let mut v_sets: Vec<HashSet<usize>> = Vec::new();
        let mut heap: BinaryHeap<Reverse<(i32, (usize, usize))>> = BinaryHeap::new();

        // Criando a floresta de conjuntos
        for i in 0..self.vertices.len() {
            v_sets.push(HashSet::from([i]));
        }

        // Ordenando as arestas
        for i in 0..self.relations.len() {
            for j in 0..self.relations.len() {
                if self.get_edge_weight(i, j).unwrap() != 0 {
                    heap.push(Reverse((self.get_edge_weight(i, j).unwrap(), (i, j))));
                }
            }
        }

        for _ in 0..heap.len() {
            let (u, v) = match heap.pop() {
                Some(Reverse((_, (u1, v1)))) => (u1, v1),
                None => (0, 0)
            };
            let (mut set1_idx, mut set2_idx): (usize, usize) = (0, 0);

            for i in 0..v_sets.len() {
                if v_sets[i].contains(&u) {
                    set1_idx = i;
                }

                if v_sets[i].contains(&v) {
                    set2_idx = i;
                }
            }

            if set1_idx != set2_idx {
                a.insert((u, v));
                let t = v_sets.remove(set2_idx);
                
                if set2_idx < set1_idx {
                    v_sets[set1_idx-1].extend(&t);
                }
                else {
                    v_sets[set1_idx].extend(&t);
                }
            }
        }

        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; self.relations.len()]; self.relations.len()];
        for (src, dst) in a.into_iter() {
            adjacency_matrix[src][dst] = self.get_edge_weight(src, dst).unwrap();
        }

        return Graph::from(self.vertices.clone(), adjacency_matrix);
    }

    // Algoritmo de Prim: inicia adicionando ao conjunto A os vertices ligados pela aresta de menor custo
    //      e após vai adicionando os vertices que tiverem menor custo e estejam sejam adjacentes aos já existentes
    pub fn get_mst_prim(&self) -> Self {
        let mut a: HashSet<usize> = HashSet::with_capacity(self.vertices.len());
        let mut heap: BinaryHeap<Reverse<(i32, (usize, usize))>> = BinaryHeap::new();
        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(self.vertices.len() - 1);

        // Pega a aresta de menor valor diferente de zero
        let mut min: (usize, usize) = (0, 0);
        for i in 0..self.vertices.len() {
            for j in 0..self.vertices.len() {
                if self.get_edge_weight(i, j).unwrap() != 0 && self.get_edge_weight(i, j).unwrap() < self.get_edge_weight(min.0, min.1).unwrap() {
                    min = (i, j);
                }
            }
        }

        // Adicionando a primeira aresta na heap
        heap.push(Reverse((self.get_edge_weight(min.0, min.1).unwrap(), min)));
        a.insert(min.0);
        let adjacents = self.get_adjacent_vertices(min.0).unwrap();

        // Adicionando as arestas para adjacentes do primeiro vertice na heap ---
        for adj_vertex in adjacents.into_iter() {
            if a.contains(&adj_vertex) {
                continue;
            }

            heap.push(Reverse((self.get_edge_weight(min.0, adj_vertex).unwrap(), (min.0, adj_vertex))));
        }

        // Itera ate que todos os vertices estejam acessiveis
        loop {
            if a.len() == self.vertices.len() {
                break;
            }

            // Remove da heap
            let (priority, src, dst) = match heap.pop() {
                Some(Reverse((prio, (sr, ds)))) => (prio, sr, ds),
                None => { continue; }
            };

            if a.contains(&dst) {
                continue;
            }

            // Insere um vertice que nao havia sido explorado ainda
            a.insert(dst);
            // Adiciona a aresta no vetor de arestas
            edges.push((src, dst));
            // Pega os adjacentes
            let adjacents = self.get_adjacent_vertices(dst).unwrap();

            for adj_vertex in adjacents.into_iter() {
                if a.contains(&adj_vertex) {
                    continue;
                }

                // Popula a heap denovo
                heap.push(Reverse((self.get_edge_weight(dst, adj_vertex).unwrap(), (dst, adj_vertex))));
            }
        }

        // Transforma tudo em um novo grafo :)
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; self.relations.len()]; self.relations.len()];
        for (src, dst) in edges.into_iter() {
            adjacency_matrix[src][dst] = self.get_edge_weight(src, dst).unwrap();
        }

        return Graph::from(self.vertices.clone(), adjacency_matrix);
    }
}

// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// https://doc.rust-lang.org/rustc/tests/index.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_graph_1() {
        let mut graph: Graph = Graph::new();

        let _ = graph.insert_vertex("A");
        let _ = graph.insert_vertex("B");
        let _ = graph.insert_vertex("C");
        let _ = graph.insert_vertex("D");
        let _ = graph.insert_vertex("E");
        let _ = graph.insert_vertex("F");
        let _ = graph.insert_edge(0, 1, 2, false);
        let _ = graph.insert_edge(0, 2, 1, false);
        let _ = graph.insert_edge(1, 3, 1, false);
        let _ = graph.insert_edge(2, 3, 3, false);
        let _ = graph.insert_edge(2, 4, 4, false);
        let _ = graph.insert_edge(4, 5, 2, false);
        let _ = graph.insert_edge(3, 5, 2, false);

        let path = graph.get_dijkstra_path(0, 5).unwrap();

        for vertex in path.iter() {
            print!("{} ", graph.get_vertex(*vertex).unwrap());
        }
    }

    #[test]
    fn exemple_graph_2() {
        let mut g2 = Graph::new();

        let _ = g2.insert_vertex("A");
        let _ = g2.insert_vertex("B");
        let _ = g2.insert_vertex("C");
        let _ = g2.insert_vertex("D");
        let _ = g2.insert_vertex("E");
        let _ = g2.insert_vertex("F");
        let _ = g2.insert_edge(0, 1, 10, false);
        let _ = g2.insert_edge(0, 3, 5, false);
        let _ = g2.insert_edge(3, 1, 3, false);
        let _ = g2.insert_edge(1, 2, 1, false);
        let _ = g2.insert_edge(3, 2, 8, false);
        let _ = g2.insert_edge(3, 4, 2, false);
        let _ = g2.insert_edge(4, 5, 6, false);
        let _ = g2.insert_edge(2, 4, 4, false);
        let _ = g2.insert_edge(2, 5, 4, false);

        let path = g2.get_dijkstra_path(0, 5).unwrap();
        let min_tree = g2.get_mst_kruskal();

        println!("{:?}", min_tree);

        for vertex in path.iter() {
            print!("{} ", g2.get_vertex(*vertex).unwrap());
        }
    }
}
