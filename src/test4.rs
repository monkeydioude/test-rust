use std::fmt::Display;

struct LulString<'a> {
    str: &'a str,
}

impl<'a> LulString<'a> {
    pub fn new(str: &'a str) -> LulString{
        LulString::<'a> { str  }
    }
}
impl Display for LulString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

pub fn main() {
    println!("{}", LulString::new("Salut les kids"))
}