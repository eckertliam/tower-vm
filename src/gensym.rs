pub struct Gensym {
    counter: u32,
}

impl Gensym {
    pub fn new() -> Self {
        Self {
            counter: 0,
        }
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn next(&mut self) -> String {
        let symbol = format!("gensym{}", self.counter);
        self.counter += 1;
        symbol
    }
}