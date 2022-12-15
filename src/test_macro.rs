use std::fmt::Debug;

trait T {
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
struct Test;

impl T for Test {
    fn name(&self) -> &'static str {
        std::any::type_name::<Test>()
    }
}

impl Debug for dyn T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

macro_rules! layer {
    ($( $x:expr),*) => {
        {
            let mut v = Vec::<Box<dyn T>>::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! test {
    ($input:expr, $($args:expr),*) => {{
        let mut v = Vec::<&str>::new();
        let w = count!($($args)*);
        let mut output: &str = "";
        let it = 0;

        $(
            let it = it + 1;
            if it < w {
                v.push($args);
            } else {
                output = $args;
            }
        )+
        ($input, v, output)
    }};
}

pub fn main() {
    // let t: Box<dyn T>;
    // format_args_nl!()
    // t = Box::new(Test{});
    // println!("{:?}", layer!(t));
    let res = test!("salut", "mon", "ami");
    println!("{:?}", res);
}