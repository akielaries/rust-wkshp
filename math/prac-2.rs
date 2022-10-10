/*
 * Return the type of variable passed in
 */
fn return_type_of<T> (_: &T) {
    println!("{}", std::any::type_name::<T>())
}


/*
 * function that will print and return the logarithm
 * based on the params passed in
 */
fn log(x: f64,b: f64) -> f64 {
    // println!("<------------------- Running log() ------------------->");
    
    if x < b {
        return 0.0;
    }
    /*
     * rethink logic. always returns whole num
     */
    let result = 1.0 + log(x / b, b);

    return result;
}

/*
 * compute the logarithm of a number given a base value
 */
fn logarithm(self, b: f64) -> f64 {
    self.ln() / b.ln()
}


// MAIN
fn main () {
    /*
    /*<---- TEST ONE ---->*/
    let num_a = 3;
    let num_b = 11;
    /*<---- TEST TWO ---->*/
    let num_c = 54;
    let num_d = 85;

    /*<---- RUN LOG ---->*/
    println!("<------------------- Running log() ------------------->");

    let n = log(num_b as f64, num_a as f64);
    println!("\nLOGARITHM log{}({}) = {:.2}\n", num_a, num_b, n as f64);

    println!("<------------------- Running log() AGAIN! ------------------->");
    let n_o = log(num_d as f64, num_c as f64);
    // 
    println!("\nLOGARITHM log{}({}) = {:.2}\n", num_c, num_d, n_o);

    /*<---- RUN INTEGRATION ---->*/
    */
    /*<---- RUN SOME DEBUGGING ---->*/
    let test_num1: i64 = 3;
    let test_num2: i64 = 11;
    let test_num3 = 11.492;

    let y: f64 = log(test_num2 as f64, test_num1 as f64);
    println!("\nDEBUGGING RESULT = {}\n", y as f64);

    println!("\nPRINTING FLOAT {}\n", test_num3);
    
    let x: f64 = logarithm(test_num2 as f64, test_num1 as f64);
    println!("\nRUST LOGARITHM = {}\n", x);


}

