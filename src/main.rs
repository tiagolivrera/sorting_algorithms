use std::env;

fn main() {
    // turbo fish ::<> args recebe argumento do tipo Vec<String>
    // equivalente a let args: Vec<String> = env::args().collect();
    // passando argumentos C-like: cargo run -- --selection_sort
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    let array = vec![5, 4, 7, -1, 10];
    let array_sorted = insertion_sort(array, 5);
    println!("{:?}", array_sorted);
}

fn insertion_sort(array: Vec<i32>, _length: usize) -> Vec<i32> {
    let mut sort = array;

    for i in 1.._length {
        for j in (1..i).rev() {
            if sort[j] < sort[j - 1] {
                let temp = sort[j - 1];
                sort[j - 1] = sort[j];
                sort[j] = temp;
            }
        }
    }
    sort
}
