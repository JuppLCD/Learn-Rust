use std::io;

fn main() {
    let n = pedir_entero('n');

    if n < 0 {
        println!("Error: n tiene que ser un numero natural");
    }else{
        let sum = suma_hasta(n);
        println!("La suma de los primeros naturales hasta {n} es {sum}");
    }
}

fn suma_hasta(n : i32) -> i32{
    let sum = n*(n + 1)/2;
    return sum;
}

fn pedir_entero(name : char) -> i32{
    let mut input = String::new();

    println!("Asignar valor de {name}: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se pudo convertir X a entero");

    return x;
}