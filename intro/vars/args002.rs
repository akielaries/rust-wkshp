fn dump_sum(a: i8, b:i8) -> i8 {
    a + b
}

fn print_sum(d: i8, e:i8) {
    let f = d + e;
    println!("Printing using print_sum :\n{} \n", f);
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
 */
fn loop_sum(g: i8,h: i8) {
    /*  
     * check that g is greater than h or else we would need to traverse,
     * or allow, in reverse
     */
    if h > g { 
        println!("Printing using loop_sum w/ vals {} - {}\n", g, h);

        // store the difference in values 
        let diff = h - g;
        println!("Difference in values: {}\n", diff);

        /*
        * check that g is greater than h or else we would need to traverse,
        * or allow, in reverse
        */
        for i in g..h {
            println!("{} + {} = {}", i, diff, h);
            println!() 


        }
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

    loop_sum(3, 11);

}



