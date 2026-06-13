mod grafo;
mod bfs;
mod dfs;
mod red_ejemplo;

use crate::red_ejemplo::{crear_red_ejemplo, mostrar_red};
use crate::bfs::{ruta_mas_corta, bfs};

fn main() {
    let g = crear_red_ejemplo();
    mostrar_red(&g);

    println!("Ejemplo: ruta más corta de 0 a 2 (BFS):");
    match ruta_mas_corta(&g, 0, 2) {
        Some(ruta) => println!("  Ruta: {:?}", ruta),
        None => println!("  No existe ruta."),
    }

    println!("\nBFS (padres) desde 0:");
    let padres = bfs(&g, 0);
    for (n, p) in padres.iter() {
        println!("  Nodo {} <- {:?}", n, p);
    }

    // Si `dfs` está implementado por el equipo, podrán invocarlo desde aquí.
}
