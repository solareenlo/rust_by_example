fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_planin = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_planin();
    fn_planin();
    fn_mut();
    fn_mut();
    fn_once();
    // fn_once();
}
