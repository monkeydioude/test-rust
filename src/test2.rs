
trait Unit {}

struct Dummy;

impl Unit for Dummy{}

fn fn_dummy() -> impl (FnOnce() -> Box<dyn Unit>) {
    || Box::new(Dummy{})
}


pub fn main() {

}