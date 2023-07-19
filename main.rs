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

    let mut g3 = match Graph::from_file(String::from("graph2.txt")) {
        Ok(g) => g,
        Err(msg) => {
            println!("{}", msg);
            Graph::new()
        }
    };

    // Imprime o grafo
    println!("{}", g3);
    
    // Imprime o grafo
    println!("{}\n", g2);
    
    // Caminho mínimo de A (0) até F (5)
    let path = g2.get_dijkstra_path(0, 5).unwrap();

    println!("Dijkstra path:");
    
    for vertex in path.iter() {
        print!(" -> {}", g2.get_vertex(*vertex).unwrap());
    }
    
    println!("\n\n");
    
    let kruskal = g3.get_mst_kruskal();
    println!("Kruskal mst:\n{}", kruskal);

    println!("\n");

    let prim = g3.get_mst_prim();
    println!("Prim mst:\n{}", prim);
}
