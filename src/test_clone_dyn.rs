trait Node: CloneNode {}

trait CloneNode {
    fn clone_box<'a>(&self) -> Box<dyn Node>;
}

impl<T> CloneNode for T
where T: Node + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn main() {
    
}