pub struct Exchange {
    from: usize,
    to: usize,
    price: f64,
    directed: bool,
}

impl Exchange {
    pub fn new(from: usize, to: usize, price: f64, directed: bool) -> Self {
        Self { from, to, price, directed }
    }

    pub fn set_price(&mut self, price: f64) -> () {
        self.price = price;
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn directed(&self) -> bool {
        self.directed
    }
}

