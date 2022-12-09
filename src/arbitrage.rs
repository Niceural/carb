use crate::exchange::Exchange;

pub struct Arbitrage {
    currency_count: usize,
    graph: Vec<Exchange>,
}

impl Arbitrage {
    //----------------------------------------- returning Self

    pub fn new() -> Self {
        Self {
            currency_count: 0,
            graph: Vec::new(),
        }
    }

    pub fn with_capacity(currency_count: usize, capacity: usize) -> Self {
        Self {
            currency_count,
            graph: Vec::with_capacity(capacity),
        }
    }

    //----------------------------------------- mut ref

    pub fn set_currency_count(&mut self, currency_count: usize) -> () {
        self.currency_count = currency_count
    }

    pub fn push_exchange(&mut self, exchange: Exchange) -> () {
        self.graph.push(exchange);
    }

    pub fn set_price(&mut self, exchange_id: usize, price: f64) -> () {
        self.graph[exchange_id].set_price(price);
    }

    //----------------------------------------- unmut ref

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn exchanges_count(&self) -> usize {
        self.graph.len()
    }

    pub fn currency_count(&self) -> usize {
        self.currency_count
    }

    pub fn capacity(&self) -> usize {
        self.graph.capacity()
    }

    pub fn execute(&self) -> () {
        self.bellman_ford(0);
    }

    fn bellman_ford(&self, start: usize) -> Vec<Option<usize>> {
        // number of vertices
        let n: usize = self.currency_count();

        // minimum cost
        let mut dists: Vec<f64> = vec![f64::INFINITY; n];
        dists[start] = 0.;

        // previous vertex
        let mut prev: Vec<Option<usize>> = vec![None; n];

        // relax edges n - 1 times
        for _ in 1..n {
            for ex in &self.graph {
                let new_cost = dists[ex.from()] + ex.price();
                if new_cost < dists[ex.to()] {
                    dists[ex.to()] = new_cost;
                    prev[ex.to()] = Some(ex.from());
                }
            }
        }

        for ex in &self.graph {
            if dists[ex.from()] + ex.price() < dists[ex.to()] {

            }
        }

        prev
    }
}

/*
// non-mutable references
impl Arbitrage {
    pub fn execute(&self) -> () {
    }

    pub fn currency_count(&self) -> usize {
        self.graph.len()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }
}
*/
