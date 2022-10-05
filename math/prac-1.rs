/*
 * testing the implementation of some different math related algorithms
 */

/*
 * This function swill print out the summation of the difference between 
 * two numbers
 * *count from ZERO*
 * e.g. 
 * A = 3
 * B = 11
 * Distance between two variables = 8
 * SUMMATION = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 = 36
 *
 */
fn summation(g: i32,h: i32) {
    println!("<------------------- Running summation() -------------------?");

    /*  
     * check that g is greater than h or else we would need to traverse,
     * or allow, in reverse
     */
    if h > g { 
        println!("\nPrinting using summation() w/ vals {} - {}\n", g, h); 

        // store the difference in values 
        let diff = h - g;
        println!("\nDifference in values: {}\n", diff);

        /* 
         * loop thru difference to take summation of
         * append diff with one to take care of 0 index. dont want to include
         * 0 in our summation
         */
        let mut sum = 0;
        for n in 1..diff + 1 { 
            sum = sum + n;

            /*
             * try to print out formatted arithmetic
             * E.G. :
             * Summation 1-8
             * 1 + 2 = 3
             * 3 + 4 = 7
             * 5 + 6 = 11
             * OR
             * store results in array and format with + in between
             * each index
             * E.G  :
             * 1 + 2 + 3 + ... + n
             */

            //println!("{} + {} = {}", n, n + 1, sum);
            //return sum;
        }
        println!("\nSummation Σ of {}-{} = {}\n", 1, diff, sum);

    }

    else {
        println!("\nERR\n");
    }
}

/*
 * function that will print and return the logarithm
 * based on the params passed in
 */
fn log(x: f64,b: f64) -> f64 {

    // println!("<------------------- Running log() ------------------->");
    
    if x < b {
        println!("\nSTMT 1 : x = {} b = {}\n", x, b);
        return 0.0;
    }
    //return 1 + log(x/b, b);
    println!("\nSTMT 2 : x = {} b = {}\n", x, b);

    // return 1.0 + log(x/b , b);
    let result = 1.0 + log(x / b, b); 

    println!("\nSTMT 3 : x = {:.2}, b = {:.2}\n", x, b);

    println!("\nSTMT 4 : RESULT = {:.2}\n", result);

    return result;
}


// MAIN
fn main () {
    /*<---- TEST ONE ---->*/
    let num_a = 3;
    let num_b = 11;
    /*<---- TEST TWO ---->*/
    let num_c = 54;
    let num_d = 85;

    /*<---- RUN SUMMATION ---->*/
    summation(num_a, num_b);
    summation(num_c, num_d);

    /*<---- RUN LOG ---->*/
    println!("<------------------- Running log() ------------------->");

    let n = log(num_b as f64, num_a as f64);
    println!("\nLOGARITHM log{}({}) = {:.2}\n", num_a, num_b, n);

    println!("<------------------- Running log() AGAIN! ------------------->");
    let n_o = log(num_d as f64, num_c as f64);
    println!("\nLOGARITHM log{}({}) = {:.2}\n", num_c, num_d, n_o);


    /*<---- RUN INTEGRATION ---->*/
}


