use crate::grafo::Grafo;

pub fn crear_red_ejemplo() -> Grafo {
    let mut g = Grafo::nuevo();

    g.agregar_nodo(0, "Miguel".to_string());
    g.agregar_nodo(1, "Karla".to_string());
    g.agregar_nodo(2, "Tania".to_string());
    g.agregar_nodo(3, "David".to_string());
    g.agregar_nodo(4, "Milagro".to_string());
    g.agregar_nodo(5, "Paco".to_string());

    g.agregar_arista(0, 1).expect("Error al agregar amistad Miguel-Karla");
    g.agregar_arista(1, 2).expect("Error al agregar amistad Karla-Tania");
    g.agregar_arista(2, 4).expect("Error al agregar amistad Tania-Milagro");
    g.agregar_arista(1, 3).expect("Error al agregar amistad Karla-David");
    g.agregar_arista(3, 5).expect("Error al agregar amistad David-Paco");
    g.agregar_arista(4, 5).expect("Error al agregar amistad Milagro-Paco");

    g
}

pub fn mostrar_red(g: &Grafo) {
    println!("=== Red social de ejemplo ===");
    let mut ids = g.obtener_nodos();
    ids.sort();
    for id in ids {
        let nombre = g.obtener_nodo(id).map(|n| n.nombre.as_str()).unwrap_or("?");
        let amigos = g.vecinos(id).unwrap();
        println!("  Usuario {} ({}): amigos = {:?}", id, nombre, amigos);
    }
    println!("=============================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodos_registrados() {
        let g = crear_red_ejemplo();
        for id in 0..6 {
            assert!(g.contiene_nodo(id), "El usuario {} debería existir", id);
        }
    }

    #[test]
    fn test_nombres_de_nodos() {
        let g = crear_red_ejemplo();
        assert_eq!(g.obtener_nodo(0).unwrap().nombre, "Miguel");
        assert_eq!(g.obtener_nodo(5).unwrap().nombre, "Paco");
    }

    #[test]
    fn test_aristas_existen() {
        let g = crear_red_ejemplo();

        let amistades_esperadas = vec![
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
            (2, 4),
            (4, 2),
            (1, 3),
            (3, 1),
            (3, 5),
            (5, 3),
            (4, 5),
            (5, 4),
        ];

        for (u, v) in amistades_esperadas {
            let amigos = g.vecinos(u).expect("Usuario no encontrado");
            assert!(amigos.contains(&v), "Debería existir amistad {} → {}", u, v);
        }
    }

    #[test]
    fn test_aristas_no_existen() {
        let g = crear_red_ejemplo();

        let pares_sin_amistad = vec![(0, 3), (1, 4), (2, 5), (0, 5)];

        for (u, v) in pares_sin_amistad {
            let amigos = g.vecinos(u).expect("Usuario no encontrado");
            assert!(!amigos.contains(&v), "No debería existir amistad {} → {}", u, v);
        }
    }

    #[test]
    fn test_grado_de_nodos() {
        let g = crear_red_ejemplo();

        let grados_esperados = vec![(0, 1), (1, 3), (2, 2), (3, 2), (4, 2), (5, 2)];

        for (id, grado_esperado) in grados_esperados {
            let grado = g.vecinos(id).unwrap().len();
            assert_eq!(grado, grado_esperado, "Usuario {} debe tener {} amigo(s)", id, grado_esperado);
        }
    }

    #[test]
    fn test_arista_nodo_inexistente_retorna_error() {
        let mut g = Grafo::nuevo();
        g.agregar_nodo(0, "Solo".to_string());
        let resultado = g.agregar_arista(0, 99);
        assert!(resultado.is_err(), "Agregar amistad a usuario inexistente debe retornar Err");
    }
}
