use std::assert;

const X : i32 =  -200;
const Y : i32 =  40;

fn main(){
    let aux;

    let mut x = X;
    let mut y = Y;

    assert!(x == X && y == Y);

    aux = x;
    x += 1;
    y += aux;

    assert!(x == X + 1 && y == X + Y);

    println!("X = {X}, \tY = {Y}\n");
    println!("x = {x}, \ty = {y}\n");
}