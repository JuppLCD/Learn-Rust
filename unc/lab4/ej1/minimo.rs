use std::io;
use std::assert;

fn main(){
    let x = pedir_entero('x');
    let y = pedir_entero('y');

    let minimo = min(x, y);

    assert!( minimo <= x);
    assert!( minimo <= y);

    println!("x = {x} \ny = {y}");
    println!("El minimo es: {minimo}");
}

fn pedir_entero(name : char) -> i32{
    let mut input = String::new();

    println!("Asignar valor a {name}: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");

    x
}

fn min(x : i32, y : i32) -> i32{
    if x < y {
        return x;
    }

    return y;
}