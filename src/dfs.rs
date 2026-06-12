use crate::grafo::Grafo;
use std::collections::HashSet;

pub fn dfs(grafo: &Grafo, inicio: usize) -> Vec<usize> {
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut orden: Vec<usize> = Vec::new();
    dfs_aux(grafo, inicio, &mut visitados, &mut orden);
    orden
}

fn dfs_aux(grafo: &Grafo, actual: usize, visitados: &mut HashSet<usize>, orden: &mut Vec<usize>) {
    if visitados.contains(&actual) {
        return;
    }
    visitados.insert(actual);
    orden.push(actual);

    if let Ok(vecinos) = grafo.vecinos(actual) {
        for &vecino in vecinos {
            if !visitados.contains(&vecino) {
                dfs_aux(grafo, vecino, visitados, orden);
            }
        }
    }
}

pub fn dfs_iterativo(grafo: &Grafo, inicio: usize) -> Vec<usize> {
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut pila: Vec<usize> = vec![inicio];
    let mut orden: Vec<usize> = Vec::new();

    while let Some(actual) = pila.pop() {
        if visitados.contains(&actual) {
            continue;
        }
        visitados.insert(actual);
        orden.push(actual);

        if let Ok(vecinos) = grafo.vecinos(actual) {
            for &vecino in vecinos.iter().rev() {
                if !visitados.contains(&vecino) {
                    pila.push(vecino);
                }
            }
        }
    }
    orden
}

pub fn existe_camino(grafo: &Grafo, inicio: usize, destino: usize) -> bool {
    dfs(grafo, inicio).contains(&destino)
}

pub fn es_conexo(grafo: &Grafo, inicio: usize) -> bool {
    let visitados = dfs(grafo, inicio);
    let total_nodos = grafo.obtener_nodos().len();
    visitados.len() == total_nodos
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grafo::Grafo;
    use crate::red_ejemplo::crear_red_ejemplo;

    #[test]
    fn test_dfs_alcanza_todos_los_nodos() {
        let g = crear_red_ejemplo();
        let orden = dfs(&g, 0);
        for id in 0..6 {
            assert!(orden.contains(&id), "DFS debería alcanzar el nodo {}", id);
        }
        assert_eq!(orden.len(), 6);
    }

    #[test]
    fn test_dfs_empieza_en_inicio() {
        let g = crear_red_ejemplo();
        let orden = dfs(&g, 0);
        assert_eq!(orden[0], 0);
    }

    #[test]
    fn test_dfs_no_repite_nodos() {
        let g = crear_red_ejemplo();
        let orden = dfs(&g, 0);
        let unicos: HashSet<usize> = orden.iter().cloned().collect();
        assert_eq!(
            orden.len(),
            unicos.len(),
            "DFS no debería visitar nodos repetidos"
        );
    }

    #[test]
    fn test_dfs_recursivo_e_iterativo_coinciden_en_conjunto() {
        let g = crear_red_ejemplo();
        let rec: HashSet<usize> = dfs(&g, 0).into_iter().collect();
        let iter: HashSet<usize> = dfs_iterativo(&g, 0).into_iter().collect();
        assert_eq!(
            rec, iter,
            "Ambas versiones deben visitar el mismo conjunto de nodos"
        );
    }

    #[test]
    fn test_existe_camino_directo() {
        let g = crear_red_ejemplo();
        assert!(existe_camino(&g, 0, 1));
    }

    #[test]
    fn test_existe_camino_varios_saltos() {
        let g = crear_red_ejemplo();
        assert!(existe_camino(&g, 0, 4));
    }

    #[test]
    fn test_no_existe_camino_nodo_aislado() {
        let mut g = Grafo::nuevo();
        g.agregar_nodo(0, "A".to_string());
        g.agregar_nodo(1, "B".to_string());
        assert!(!existe_camino(&g, 0, 1));
    }

    #[test]
    fn test_es_conexo_red_ejemplo() {
        let g = crear_red_ejemplo();
        assert!(es_conexo(&g, 0), "La red de ejemplo debería ser conexa");
    }

    #[test]
    fn test_no_es_conexo_con_nodo_aislado() {
        let mut g = Grafo::nuevo();
        g.agregar_nodo(0, "A".to_string());
        g.agregar_nodo(1, "B".to_string());
        g.agregar_nodo(2, "C".to_string());
        g.agregar_arista(0, 1).unwrap();
        assert!(!es_conexo(&g, 0));
    }
}
