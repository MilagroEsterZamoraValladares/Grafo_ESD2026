mod grafo;
mod bfs;
mod dfs;
mod red_ejemplo;

use crate::red_ejemplo::{crear_red_ejemplo, mostrar_red};
use crate::bfs::{ruta_mas_corta, bfs};
use crate::dfs::{dfs, dfs_iterativo, existe_camino, es_conexo};
use crate::grafo::Grafo;

fn nombre_usuario(g: &Grafo, id: usize) -> String {
    g.obtener_nodo(id)
        .map(|n| n.nombre.clone())
        .unwrap_or_else(|| format!("Usuario_{}", id))
}

fn ruta_nombres(g: &Grafo, ruta: &[usize]) -> Vec<String> {
    ruta.iter().map(|&id| nombre_usuario(g, id)).collect()
}

fn main() {
    let g = crear_red_ejemplo();
    mostrar_red(&g);

    println!("Ejemplo: ruta más corta de Miguel a Tania (BFS):");
    match ruta_mas_corta(&g, 0, 2) {
        Some(ruta) => {
            let nombres = ruta_nombres(&g, &ruta);
            println!("  Ruta ids: {:?}", ruta);
            println!("  Ruta nombres: {:?}", nombres);
        }
        None => println!("  No existe ruta."),
    }

    println!("\nBFS (padres) desde Miguel:");
    let padres = bfs(&g, 0);
    for (n, p) in padres.iter() {
        let hijo = nombre_usuario(&g, *n);
        let padre = p.map(|id| nombre_usuario(&g, id)).unwrap_or_else(|| "Ninguno".to_string());
        println!("  {} <- {}", hijo, padre);
    }

    println!("\nDFS (recursivo) desde Miguel:");
    let orden_rec = dfs(&g, 0);
    let orden_rec_nombres = ruta_nombres(&g, &orden_rec);
    println!("  Orden recursivo ids: {:?}", orden_rec);
    println!("  Orden recursivo nombres: {:?}", orden_rec_nombres);

    println!("\nDFS (iterativo) desde Miguel:");
    let orden_iter = dfs_iterativo(&g, 0);
    let orden_iter_nombres = ruta_nombres(&g, &orden_iter);
    println!("  Orden iterativo ids: {:?}", orden_iter);
    println!("  Orden iterativo nombres: {:?}", orden_iter_nombres);

    println!("\nComprobaciones DFS:");
    println!("  Existe camino Miguel -> Paco? {}", existe_camino(&g, 0, 5));
    println!("  Grafo es conexo desde Miguel? {}", es_conexo(&g, 0));
}

