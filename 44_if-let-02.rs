enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    let d = Foo::Qux(101);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 101) = d {
        println!("d is {}", value);
    }

    // Error
    // if Foo::Bar == a {
    //     println!("a is foobar");
    // }
}
