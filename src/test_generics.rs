trait Node {}

trait Output {
    fn wesh(&self);
}

struct NodeA;
struct NodeB;

impl Node for NodeA {}
impl Node for NodeB {}
impl Output for NodeB{
    fn wesh(&self) {
        println!("wesh alors");
    }
}

struct TestFactory<T> 
where T: Node {
    builder: Option<Box<dyn Fn() -> T>>,
}

impl<T> TestFactory<T>
where T: Node {
    pub fn build(&self) -> T {
        self.builder.as_ref().unwrap()()
    }

    pub fn new(b: Box<dyn Fn() -> T>) -> Self {
        Self{
            builder: Some(b),
        }
    }
}

fn call_node_only(trial: impl Node) {}
fn call_node_and_output(trial: impl Node + Output) {
    trial.wesh();
}

pub fn main() {
    let t: [u32; 4] = [0, 0, 0, 0];
    println!("{:?}", t);

    let t1 = TestFactory::<NodeA>::new(Box::new(|| -> NodeA {
        NodeA{}
    })).build();
    let t2 = TestFactory::<NodeB>::new(Box::new(|| -> NodeB {
        NodeB{}
    })).build();

    call_node_only(t1);
    call_node_and_output(t2);
}