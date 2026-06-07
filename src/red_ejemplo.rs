use crate::grafo::Grafo;
pub fn crear_red_ejemplo() -> Grafo {
    let mut g = Grafo::nuevo();

    // Registrar nodos
    g.agregar_nodo(0, "Nodo_A".to_string());
    g.agregar_nodo(1, "Nodo_B".to_string());
    g.agregar_nodo(2, "Nodo_C".to_string());
    g.agregar_nodo(3, "Nodo_D".to_string());
    g.agregar_nodo(4, "Nodo_E".to_string());
    g.agregar_nodo(5, "Nodo_F".to_string());

    // Fila superior
    g.agregar_arista(0, 1).expect("Error al agregar arista 0-1");
    g.agregar_arista(1, 2).expect("Error al agregar arista 1-2");

    // Fila inferior
    g.agregar_arista(3, 4).expect("Error al agregar arista 3-4");
    g.agregar_arista(4, 5).expect("Error al agregar arista 4-5");

    // Conexiones verticales
    g.agregar_arista(0, 3).expect("Error al agregar arista 0-3");
    g.agregar_arista(2, 5).expect("Error al agregar arista 2-5");

    g
}

/// Imprime los vecinos de cada nodo en orden, para verificar visualmente la red.
pub fn mostrar_red(g: &Grafo) {
    println!("=== Red de ejemplo ===");
    let mut ids = g.obtener_nodos();
    ids.sort(); // orden consistente al imprimir
    for id in ids {
        let nombre = g.obtener_nodo(id).map(|n| n.nombre.as_str()).unwrap_or("?");
        let vecinos = g.vecinos(id).unwrap();
        println!("  Nodo {} ({}): vecinos = {:?}", id, nombre, vecinos);
    }
    println!("======================");
}

// ─── Tests básicos ────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodos_registrados() {
        let g = crear_red_ejemplo();
        for id in 0..6 {
            assert!(g.contiene_nodo(id), "El nodo {} debería existir", id);
        }
    }

    #[test]
    fn test_nombres_de_nodos() {
        let g = crear_red_ejemplo();
        assert_eq!(g.obtener_nodo(0).unwrap().nombre, "Nodo_A");
        assert_eq!(g.obtener_nodo(5).unwrap().nombre, "Nodo_F");
    }

    #[test]
    fn test_aristas_existen() {
        let g = crear_red_ejemplo();

        // Grafo no dirigido → cada arista aparece en ambas direcciones
        let aristas_esperadas = vec![
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
            (3, 4),
            (4, 3),
            (4, 5),
            (5, 4),
            (0, 3),
            (3, 0),
            (2, 5),
            (5, 2),
        ];

        for (u, v) in aristas_esperadas {
            let vecinos = g.vecinos(u).expect("Nodo no encontrado");
            assert!(vecinos.contains(&v), "Debería existir arista {} → {}", u, v);
        }
    }

    #[test]
    fn test_aristas_no_existen() {
        let g = crear_red_ejemplo();

        let aristas_ausentes = vec![(0, 2), (1, 4), (3, 5), (0, 5)];

        for (u, v) in aristas_ausentes {
            let vecinos = g.vecinos(u).expect("Nodo no encontrado");
            assert!(
                !vecinos.contains(&v),
                "No debería existir arista {} → {}",
                u,
                v
            );
        }
    }

    #[test]
    fn test_grado_de_nodos() {
        let g = crear_red_ejemplo();

        // Cada nodo en esta topología tiene exactamente grado 2
        for id in 0..6 {
            let grado = g.vecinos(id).unwrap().len();
            assert_eq!(grado, 2, "Nodo {} debe tener grado 2", id);
        }
    }

    #[test]
    fn test_arista_nodo_inexistente_retorna_error() {
        let mut g = Grafo::nuevo();
        g.agregar_nodo(0, "Solo".to_string());
        let resultado = g.agregar_arista(0, 99);
        assert!(
            resultado.is_err(),
            "Agregar arista a nodo inexistente debe retornar Err"
        );
    }
}
