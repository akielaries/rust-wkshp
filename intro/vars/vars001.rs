let a; // Declaration; without data type
a = 5; // Assignment

let b: i8; // Declaration; with data type 
b = 5;

let t = true;        // Declaration + assignment; without data type
let f: bool = false; // Declaration + assignment; with data type

let (x, y) = (1, 2); // x = 1 and y = 2

let mut z = 5;
z = 6;

let z = { x + y }; // z = 3

let z = {
    let x = 1;
    let y = 2;

    x + y
}; // z = 3

