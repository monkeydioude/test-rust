type CloseTest<'a> = Box<dyn Fn() -> &'a dyn T1>; 
type CloseTestPtr<'a> = &'a dyn Fn() -> &'a dyn T1; 

trait T1 {
    fn print(&self, t: &str);
}


struct T1Struct {
    func: fn(&str) -> String,
}

impl T1 for T1Struct {
    fn print(&self, t: &str) {
        println!("{}", t);
    }
}

// fn test_builder(units: Vec<F32>) -> 

// fn return_t1<'a>(cb: &'static dyn T1) -> CloseTest<'a> {
//     Box::new(move || -> &'a dyn T1 {cb})
// }

fn test_str(str: &str) -> String {
    str.to_string()
}

fn pass_fn(func: fn(&str) -> String) -> T1Struct {
    T1Struct { func }
}

pub fn main() {
    (pass_fn(test_str).func)("salut");
}