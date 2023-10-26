use std::io;

fn main(){
    let x = pedir_entero('n');

    imprime_entero('n', x);
}

fn pedir_entero(name : char) -> i32 {
    let mut x = String::new();

    println!("Valor de {name}: ");
    io::stdin().read_line(&mut x).expect("Error al leer la entrada");

    let x: i32 = x.trim().parse().expect("No se pudo convertir a entero el input");

    x
}

fn imprime_entero(name : char, x : i32){
    println!("{name} = {x}");
}