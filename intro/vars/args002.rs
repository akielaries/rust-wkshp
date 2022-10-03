/*
 * file to test out using functions, passing around variables, and some more
 */

/*
 * Lets test using functions, for this one we will set the result
 * value in main and return usin -> and specify the type
 * i8 = 8-bit signed integer
 * i32 = 32-bit signed integer type
 */
fn dump_sum(a: i32, b:i32) -> i32 {
    println!("<------------------- Running dump_sum() ------------------->");
    a + b
}

/*
 * function to print the sum of two numbers passed in
 */
fn print_sum(d: i32, e:i32) {
    println!("<------------------- Running print_sum() ------------------->");
    let f = d + e;
    println!("Printing using print_sum()\n");
    println!("{} + {} = {}", d, e , f);
}

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

fn main() {
    // for test one
    let num_a = 3;
    let num_b = 11;
    // for test two
    let num_c = 54;
    let num_d = 85;

    /* 
     * call dump_sum which adds two 8-bit signed integers and assigns the 
     * result to c
     */
    let num_e = dump_sum(num_a, num_b);
    println!("{} + {} = {}", num_a, num_b, num_e);

    let num_f = dump_sum(num_c, num_d);
    println!("{} + {} = {}", num_c, num_d, num_f);

    /*-------------------------*/
    print_sum(num_a, num_b);
    print_sum(num_c, num_d);
    
    summation(num_a, num_b);
    summation(num_c, num_d);

}



