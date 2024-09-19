use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("The type of the variable is: {}", type_name::<T>());
}

fn main() {
    let x = 1;
    let y = 1.1;
    let z = "hello";

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
}
