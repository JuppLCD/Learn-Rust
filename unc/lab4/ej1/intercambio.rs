use std::io;
use std::assert;

fn main(){
    let mut x = pedir_entero('x');
    let mut y = pedir_entero('y');

    let aux = x;

    assert!(aux == x);

    x = y;
    y = aux;

    assert!(aux == y);

    println!("x = {x}, y = {y}");
}

fn pedir_entero(name : char) -> i32{
    let mut input = String::new();

    println!("Asignar valor a {name}: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");

    x
}