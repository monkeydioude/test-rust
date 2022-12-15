struct Test;

impl Test {
    pub fn echo(self) -> Self {
        self
    }

    pub fn echo_ptr(&self) -> &Self {
        self
    }
}

fn new() -> Test {
    Test
}

pub fn main() {
    let t = new();
    t.echo_ptr();
    t.echo();
}