use std::assert;

const X : i32 = 7;
const Y : i32 = 3;
const Z : i32 = 5;

fn main() {
    let mut x = X;
    let mut y = Y;
    let mut z = Z;

    let aux1; let aux2;

    assert!(x == X && y == Y && z == Z);

    aux1 = x;
    aux2 = y;

    x = aux2;
    y = aux2 + aux1 + z;
    z = aux1 + aux2;

    assert!(x == Y && z == Y + X && y == Y + X + Z);

    println!("X = {X},    Y = {Y},    Z = {Z}\n");
    println!("x = {x},    y = {y},    z = {z}\n");
}