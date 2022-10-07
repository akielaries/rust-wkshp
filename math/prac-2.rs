/*
 * function that will print and return the logarithm
 * based on the params passed in
 */
fn log(x: f64,b: f64) -> f64 {
    // println!("<------------------- Running log() ------------------->");
    
    if x < b {
        println!("\nSTMT 1 : x = {:.2} b = {:.2}\n", x, b);
        return 0.0;
    }
    //return 1 + log(x/b, b);
    println!("\nSTMT 2 : x = {} b = {}\n", x, b);

    // return 1.0 + log(x/b , b);
    let result = 1.0 + log(x / b, b);

    println!("\nSTMT 3 : x = {:.2}, b = {:.2}\n", x, b);

    println!("\nSTMT 4 : RESULT = {:.2}\n", result as f64);

    return result;
}


// MAIN
fn main () {
    /*<---- TEST ONE ---->*/
    let num_a = 3_i64;
    let num_b = 11_i64;
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

    /*<---- RUN SOME DEBUGGING ---->*/
    

}

