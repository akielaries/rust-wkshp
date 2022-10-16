/*
 * playing with pointers in rust
 */

fn plus_one(a: i32) -> i32 {
    let b = a + 1;
    return b;
}

fn square(c: i64) -> i64 {
    let sqr = c * c;
    return sqr;
    
}

fn main() {
    /*without ptrs*/
    let p1 = plus_one;
    let x = p1(5);
    
    println!("{}", x);

    /*with ptrs*/
    let p1_i: fn(i32) -> i32 = plus_one;
    let x_i = p1_i(7);

    println!("{}", x_i);

    

}


