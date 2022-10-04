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
    println!("<------------------- Running summation() ------------------->");

    /*  
     * check that g is greater than h or else we would need to traverse,
     * or allow, in reverse
     */
    if h > g { 
        println!("Printing using loop_sum() w/ vals {} - {}\n", g, h); 

        // store the difference in values 
        let diff = h - g;
        println!("Difference in values: {}\n", diff);

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
        println!("\nSummation Σ of {}-{} = {}", 1, diff, sum);

    }

    else {
        println!("ERR");
    }
}

/*
 * function that will print and return the logarithm
 * based on the params passed in
 */
fn log(x: f32,b: f32) -> f32 {

    // println!("<------------------- Running log() ------------------->");
    
    if x < b {
        println!("ERR {}", x);
        return 0;
    }
    return 1 + log(x/b, b);
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

    /*<---- RUN INTEGRATION ---->*/
    let n = log(num_b as f32, num_a as f32);
    println!("LOGARITHM log{}({}) = {}", num_a, num_b, n);

}


