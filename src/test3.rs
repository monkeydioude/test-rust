#[derive(Debug)]
enum Test {
    F64(f64),
    Null,
}

impl Test {
    fn f64(&self) -> Result<f64, String> {
        if let Test::F64(v) = self {
            return Ok(*v)
        }
        Err("Not an f64".to_string())
    }
}

fn multiply(t1: Test) -> Result<Test, String> {
    // match t1.f64() {
    //     Ok(v) => Test::F64(v*2.),
    //     _ => Test::Null,
    // }
    Ok(Test::F64(t1.f64()?*2.))
}


pub fn main() {
    let t1 = Test::F64(6.9);
    println!("{:?}", multiply(t1));
}