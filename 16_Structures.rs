#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rec: Rectangle) -> f32 {
    let w = (rec.bottom_right.x - rec.top_left.x).abs();
    let h = (rec.top_left.y - rec.bottom_right.y).abs();
    return w * h;
}

fn square(p: Point, len: f32) -> Rectangle {
    let Point { x, y } = p;
    let rec = Rectangle {
        top_left: p,
        bottom_right: Point {
            x: x + len,
            y: y - len,
        },
    };
    return rec;
}

fn main() {
    let name = String::from("Peter");
    let age = 20;
    let peter = Person { name, age };
    println!("{:?}", peter);
    println!();

    let point: Point = Point { x: 10.0, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    println!();

    let bottom_right = Point { x: 5.0, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    println!();

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!(
        "({}, {}), ({}, {})",
        _rectangle.top_left.x,
        _rectangle.top_left.y,
        _rectangle.bottom_right.x,
        _rectangle.bottom_right.y
    );
    println!("Area: {}", rect_area(_rectangle));
    println!();

    let _unit = Unit;
    let pair = Pair(1, 1.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    println!();

    let square = square(point, 2.0);
    println!("Area: {}", rect_area(square));
}
