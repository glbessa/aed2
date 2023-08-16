mod hash_table;
//mod graph;

use hash_table::HashTable;
//use graph::Graph;

fn main() {
    let mut h: HashTable<u8> = HashTable::with_capacity(30);
    match h.put(&String::from("gabriel"), 10) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    };

    println!("{}", h.get(&String::from("gabriel")).unwrap())
}

/*
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
*/