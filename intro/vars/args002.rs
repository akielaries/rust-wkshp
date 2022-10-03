/*
 * Lets test using functions, for this one we will set the result
 * value in main and return usin -> and specify the type
 * i8 = 8-bit signed integer
 */
fn dump_sum(a: i8, b:i8) -> i8 {
    println!("<------------------- Running dump_sum() ------------------->");
    a + b
}

/*
 * function to print the sum of two numbers passed in
 */
fn print_sum(d: i8, e:i8) {
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
fn summation(g: i8,h: i8) {
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
            //println!("{} + {} = {}", i, 1, sum);
            //return sum;
        }
        println!("Summation Σ of {}-{} = {}", 1, diff, sum);

    }

    else {
        println!("ERR");
    }
}

fn main() {
    let c = dump_sum(9, 1);
    
    println!("Printing using dump_sum(): \n{}\n", c);

    /*-------------------------*/
    print_sum(10, 2);
    
    summation(3, 11);
    
    summation(1, 12);

}



