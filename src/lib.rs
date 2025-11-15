use rand::Rng;
pub struct Cell {
    alive: bool,
    age: u32,
}

impl Cell {
    pub fn new(alive: bool, age: u32) -> Cell {
        Cell { alive, age }
    }
    pub fn init_cell() -> Cell {
        let mut thr = rand::rng();
        Cell::new(thr.random_bool(0.5), 0)
    }
    pub fn alive(&self) -> bool {
        self.alive
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn check_rules(&mut self, curr_state: bool, neighbours: u32) {
        if curr_state {
            if neighbours == 2 || neighbours == 3 {
                self.alive = true;
                self.age += 1;
            } else {
                self.alive = false;
                self.age = 0;
            }
        } else {
            if neighbours == 3 {
                self.alive = true;
                self.age += 1
            } else {
                self.alive = false;
                self.age = 0;
            }
        }
    }
}
