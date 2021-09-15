struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// データは一度にいくつでもイミュータブルに借用することができるが，
// その間オリジナルのデータをミュータブルに借用することはできない．
// 一方でミュータブルな借用は一度に一つしか借用することができない．
// オリジナルのデータをもう一度借用できるのは，
// ミュータブルな参照が最後に使われた場所よりあとでなければならない．
fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;
    mutable_borrow.x = 1;
    mutable_borrow.y = 2;
    mutable_borrow.z = 3;

    // let y = &point.y;
    // println!("Point z coordinates is {}", point.z);

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    println!("Point z coordinates is {}", point.z);

    let new_borrowd_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        new_borrowd_point.x, new_borrowd_point.y, new_borrowd_point.z
    );
}
