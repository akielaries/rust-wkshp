/*
 * playing with pointers in rust
 */

fn plus_one(a: i32) -> i32 {
    let b = a + 1;
    return b;
}

fn plus_n(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;

}

fn square(c: i64) -> i64 {
    let sqr = c * c;
    return sqr;
    
}

fn main() {
    /*without ptrs*/
    let p1 = plus_one;
    let x = p1(5);
    println!("plus_one() w/o ptrs   - {}", x);

    /*with ptrs*/
    let p1_i: fn(i32) -> i32 = plus_one;
    let x_i = p1_i(7);
    println!("plus_one() w/ ptrs    - {}", x_i);

    /*w/o ptrs*/
    let pn = plus_n;
    let x_n = pn(7, 6);
    println!("plus_n() w/o ptrs     - {}", x_n);

    /*w/ ptrs*/
    let pp: fn(i32, i32) -> i32 = plus_n;
    let pp_i = pp(4, 6);
    println!("plus_n w/ ptrs        - {}", pp_i);

    /*playing with squares*/
    let sq = square;
    let x_s = sq(4);
    println!("square() w/o ptrs     - {}", x_s);

    let sq_p: fn(i64) -> i64 = square;
    let x_p = sq_p(3);
    println!("square() w/ ptrs      - {}", x_p);

}


