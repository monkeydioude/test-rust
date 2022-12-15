#[derive(Copy, Clone)]
struct Test {
    v: i32
}

impl Test {
    pub fn set(&mut self, v: i32) -> Self {
        self.v = v;
        *self
    }

    pub fn get(&self) -> i32 {
        self.v
    }
}

fn get_v(v: &i32) -> i32 {
    *v
}

fn new() -> Test {
    Test {v: 0}
}

pub fn main() {
    // let t = &1;

    // println!("{}", get_v(t));

    let mut t = new();

    t.set(2);
    println!("{}", t.get());
}