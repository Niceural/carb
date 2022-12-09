#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}

impl Edge {
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight, }
    }
}

pub struct WeightedAdjacencyList {
    vertices: Vec<Vec<Edge>>,
}

impl WeightedAdjacencyList {
    //---------------------------------------------- returning Self

    pub fn new() -> Self {
        Self { vertices: Vec::new() }
    }

    pub fn with_size(vertex_count: usize) -> Self {
        Self { vertices: vec![vec![]; vertex_count] } 
    }

    //---------------------------------------------- getters
    
    //---------------------------------------------- getters

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}
