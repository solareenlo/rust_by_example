enum VeryVerboseEnumOfThinsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThinsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThinsToDoWithNumbers;

fn main() {
    let mut _x = Operations::Add;
    println!("{}", _x.run(10, 4));

    let _x = Operations::Subtract;
    println!("{}", _x.run(10, 4));
}
