use std::io;
use std::assert;


fn main(){
    let n = pedir_entero('n');

    hola_hasta(n);
}

fn pedir_entero(name : char) -> i32{
    let mut input = String::new();

    println!("Ingrese el valor de {name}: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");

    x
}

fn hola_hasta(n : i32){
    assert!(n > 0, "n es negativo!");

    let mut i = 0;

    while i < n {
        println!("hola");

        i += 1;
    }

}