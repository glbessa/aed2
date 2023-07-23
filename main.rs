/* 
 * Autor: Gabriel Leite Bessa
 * Data: 19/07/2023
 * 
 * Como executar: rustc main.rs ; ./main
 *
 */

mod graph;

use graph::Graph;

fn main() {
    let mut g2 = Graph::from_file(String::from("graph1.txt")).unwrap();

    let mut g3 = Graph::from_file(String::from("tsp3_1194.txt")).unwrap();

    let mut g4 = Graph::from_file(String::from("graph3.txt")).unwrap();

    println!("{}", g4);

    //let kruskal = g4.get_mst_kruskal();
    println!("Kruskal mst:\n{:?}", kruskal.relations);
    for t in kruskal.relations.into_iter() {
        println!("{:?}", t);
    }

    /*
    // Imprime o grafo
    println!("{}", g3);
    
    // Imprime o grafo
    println!("{}\n", g2);
    
    // Caminho mínimo de A (0) até F (5)
    let path = g2.get_dijkstra_path(0, 5).unwrap();

    println!("Dijkstra path:");
    
    for i in 0..path.len() {
        if i != 0 {
            print!(" -> ")
        }
        print!("{}", g2.get_vertex(path[i]).unwrap());
    }
    
    println!("\n\n");
    
    let kruskal = g3.get_mst_kruskal();
    println!("Kruskal mst:\n{}", kruskal);

    println!("\n");

    let prim = g3.get_mst_prim();
    println!("Prim mst:\n{}", prim);*/
}
