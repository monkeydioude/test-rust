#[derive(PartialEq)]
enum Test {
    A(i32),
    B(f32),
}

impl Test {
    pub fn type_eq(&self, against: Test) -> bool {
        match (self, against) {
            (Test::A(_), Test::A(_)) => true,
            // (Test::A(_), Test::B(_)) => false,
            // (Test::B(_), Test::A(_)) => false,
            (Test::B(_), Test::B(_)) => true,
            (_, _) => false
        }
    }
}

pub fn main() {
    let a1 = Test::A(2);
    let a2 = Test::A(3);
    println!("{}", a1.type_eq(a2));
}