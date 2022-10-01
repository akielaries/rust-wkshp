// function for printing 
fn run_prints() {
    println!("{}, {}!", "rust", "lang");
    println!("{0}, {1}!", "rust_", "lang_");
    println!("{param_one}, {param_two}!", 
             param_one = "rust__", param_two = "lang__");

    // 2 variable bindings to declare and initialize in 1 line
    let (param_one, param_two) = ("rust___", "lang___");
    println!("{param_one}, {param_two}!");
    
    println!("{:?}", [9, 222, 17]);
    println!("{:#?}", [9, 222, 17]):

    // format() store formatted string
    let x = format!("{}, {}!", "rust____", "lang____");
    println!("{}", x);

    print!("rust lang w/o a carriage return");
    print!("rust lang w/ a carriage return\n");

}

// main function, lets see how easy this part is
fn main() {
    run_prints();

}

