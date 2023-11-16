use std::io;
use std::assert;

fn main(){
    let x = pedir_entero('x');

    let x = abs(x);

    assert!( x > 0);

    println!("x = {x}");
}

fn pedir_entero(name : char) -> i32{
    let mut input = String::new();

    println!("Ingrese el valor de {name}: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");

    x
}

fn abs(x : i32) -> u32{
    if x < 0 {
        return ((-1) * x) as u32;
    }

    return x as u32;
}