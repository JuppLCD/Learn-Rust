use std::io;

fn main() {
    let mut input = String::new();

    println!("Ingrese el valor de X:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");
    input.clear();

    println!("Ingrese el valor de Y:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let y: i32 = input.trim().parse().expect("No se pudo convertir Y a entero");
    input.clear();

    println!("Ingrese el valor de Z:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let z: i32 = input.trim().parse().expect("No se pudo convertir Z a entero");
    input.clear();


    let e1: i32 = x + y + 1;
    let e2: i32 = z * z + y * 45 - 15 * x;
    let e3: bool = y - 2 == (x * 3 + 1) % 5;
    let e4: i32 = y / 2 * x;
    let e5: bool = y < x * z;

    println!("El resultado de la expresión N°1 es: {e1}");
    println!("El resultado de la expresión N°2 es: {e2}");
    println!("El resultado de la expresión N°3 es: {e3}");
    println!("El resultado de la expresión N°4 es: {e4}");
    println!("El resultado de la expresión N°5 es: {e5}");
}


/*
Ejecucion
-- a        =>      (x => 7, y => 3, z => 5)
El resultado de la expresion N°1 es: 11
El resultado de la expresion N°2 es: 55
El resultado de la expresion N°3 es: 0      (Bool => False)
El resultado de la expresion N°4 es: 7
El resultado de la expresion N°5 es: 1      (Bool => True)

-- b        =>      (x => 1, y => 10, z => 8)
El resultado de la expresion N°1 es: 12
El resultado de la expresion N°2 es: 499
El resultado de la expresion N°3 es: 0      (Bool => False)
El resultado de la expresion N°4 es: 5
El resultado de la expresion N°5 es: 0      (Bool => False)
*/
