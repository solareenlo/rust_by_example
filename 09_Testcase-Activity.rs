use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (cnt, v) in vec.iter().enumerate() {
            write!(f, "{}: {}", cnt, v)?;
            if cnt != vec.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
