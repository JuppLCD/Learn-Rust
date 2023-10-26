use std::io;

fn main(){
    let x = pedir_booleano('p');
    imprime_booleano('p', x);
}

fn pedir_booleano(name : char) -> bool{
    let temp = scanf_propio(name);

    let x = temp == 0;

    x
}

fn imprime_booleano(name : char, x : bool){
    println!("{name} = {x}");
}

fn scanf_propio(name : char) -> i32{
    let mut x = String::new();

    println!("Valor de {}:", name);
    io::stdin().read_line(&mut x).expect("Error al leer la entrada");

    let x: i32 = x.trim().parse().expect("No se pudo convertir a entero el input");

    x
}