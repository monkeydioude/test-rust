use std::fmt::Display;

struct lf1 {
    pub msg: Str,
}

struct Str {
    pub str: &'static str,   
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.str)
    }
}

impl Display for lf1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

fn get_lf1<'a>(msg: &'static str) -> lf1 {
    lf1{
        msg: Str {
            str: msg.clone()
        }
    }
}

fn get_lf1_cb<'a>(msg: &'static str) -> Box<dyn Fn() -> lf1> {
    Box::new(move || {
        get_lf1(msg)
    })
}

pub fn main() {
    let mut lf: lf1 = lf1{msg: Str{str: "heyooo"}};
    {
        lf = get_lf1_cb("stay cheeki breeki")();
    }
    println!("{}", lf);
}