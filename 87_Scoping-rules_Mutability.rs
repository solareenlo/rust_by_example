#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year += 1;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "solareenlo",
        title: "Hello world",
        year: 2020,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
    borrow_book(&immutabook);
    borrow_book(&mutabook);
}
