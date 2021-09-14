fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 4));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 4));
    println!();

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2));
    println!();

    let vec = vec![1, 2, 3, 4, 5, 6];
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    let index_of_first_negative_number = vec.iter().position(|x| x < &0);

    println!("{:?}", index_of_first_even_number);
    println!("{:?}", index_of_first_negative_number);
}
