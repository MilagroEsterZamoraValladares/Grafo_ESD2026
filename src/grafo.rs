use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Nodo {
    pub id: usize,
    pub nombre: String,
}

pub struct Grafo {
    nodos: HashMap<usize, Nodo>,
    adyacencia: HashMap<usize, Vec<usize>>,
}

impl Grafo {
    pub fn nuevo() -> Self {
        Grafo {
            nodos: HashMap::new(),
            adyacencia: HashMap::new(),
        }
    }

    pub fn agregar_nodo(&mut self, id: usize, nombre: String) {
        self.nodos.insert(id, Nodo { id, nombre });
        self.adyacencia.entry(id).or_insert(Vec::new());
    }

    pub fn agregar_arista(&mut self, id1: usize, id2: usize) -> Result<(), String> {
        if !self.nodos.contains_key(&id1) {
            return Err(format!("Nodo {} no existe", id1));
        }
        if !self.nodos.contains_key(&id2) {
            return Err(format!("Nodo {} no existe", id2));
        }

        self.adyacencia.entry(id1).or_insert(Vec::new()).push(id2);
        self.adyacencia.entry(id2).or_insert(Vec::new()).push(id1);
        Ok(())
    }

    pub fn vecinos(&self, id: usize) -> Result<&Vec<usize>, String> {
        self.adyacencia.get(&id).ok_or("Nodo no existe".to_string())
    }

    pub fn obtener_nodos(&self) -> Vec<usize> {
        self.nodos.keys().cloned().collect()
    }

    pub fn obtener_nodo(&self, id: usize) -> Option<&Nodo> {
        self.nodos.get(&id)
    }

    pub fn contiene_nodo(&self, id: usize) -> bool {
        self.nodos.contains_key(&id)
    }
}
