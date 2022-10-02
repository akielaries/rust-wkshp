fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}

fn plus_uno(a: i32) {
    println!("append sum: {}", a + 1);
}

fn main() {
    print_sum(3, 4);
    
    plus_uno(7);
}

