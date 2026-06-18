use crate::grafo::Grafo;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(grafo: &Grafo, inicio: usize) -> HashMap<usize, Option<usize>> {
    let mut padres: HashMap<usize, Option<usize>> = HashMap::new();
    let mut visitados: HashSet<usize> = HashSet::new();
    let mut cola: VecDeque<usize> = VecDeque::new();

    padres.insert(inicio, None);
    visitados.insert(inicio);
    cola.push_back(inicio);

    while let Some(actual) = cola.pop_front() {
        if let Ok(vecinos) = grafo.vecinos(actual) {
            for &vecino in vecinos {
                if !visitados.contains(&vecino) {
                    visitados.insert(vecino);
                    padres.insert(vecino, Some(actual));
                    cola.push_back(vecino);
                }
            }
        }
    }

    padres
}

pub fn reconstruir_camino(
    padres: &HashMap<usize, Option<usize>>,
    inicio: usize,
    destino: usize,
) -> Option<Vec<usize>> {
    if !padres.contains_key(&destino) {
        return None;
    }

    let mut camino = Vec::new();
    let mut actual = destino;

    loop {
        camino.push(actual);
        if actual == inicio {
            break;
        }
        match padres.get(&actual) {
            Some(Some(padre)) => actual = *padre,
            _ => return None,
        }
    }

    camino.reverse();
    Some(camino)
}

pub fn ruta_mas_corta(grafo: &Grafo, inicio: usize, destino: usize) -> Option<Vec<usize>> {
    let padres = bfs(grafo, inicio);
    reconstruir_camino(&padres, inicio, destino)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grafo::Grafo;
    use crate::red_ejemplo::crear_red_ejemplo;

    #[test]
    fn test_ruta_directa() {
        let g = crear_red_ejemplo();
        let ruta = ruta_mas_corta(&g, 0, 1);
        assert_eq!(ruta, Some(vec![0, 1]));
    }

    #[test]
    fn test_ruta_varios_saltos() {
        let g = crear_red_ejemplo();
        let ruta = ruta_mas_corta(&g, 0, 2);
        assert_eq!(ruta, Some(vec![0, 1, 2]));
    }

    #[test]
    fn test_ruta_mismo_nodo() {
        let g = crear_red_ejemplo();
        let ruta = ruta_mas_corta(&g, 3, 3);
        assert_eq!(ruta, Some(vec![3]));
    }

    #[test]
    fn test_ruta_no_existe() {
        let mut g = Grafo::nuevo();
        g.agregar_nodo(0, "A".to_string());
        g.agregar_nodo(1, "B".to_string());
        let ruta = ruta_mas_corta(&g, 0, 1);
        assert_eq!(ruta, None);
    }

    #[test]
    fn test_longitud_minima() {
        let g = crear_red_ejemplo();
        let ruta = ruta_mas_corta(&g, 1, 4).unwrap();
        assert_eq!(ruta.len(), 3);
    }

    #[test]
    fn test_bfs_alcanza_todos_los_nodos() {
        let g = crear_red_ejemplo();
        let padres = bfs(&g, 0);
        for id in 0..6 {
            assert!(padres.contains_key(&id), "BFS no alcanzó nodo {}", id);
        }
    }
}
