use std::io;

fn main() {
    const N : usize = 5;
    let mut arr : [i32; 5] = [0; N];

    pedir_arreglo(&mut arr);
    imprimir_arreglo(&arr);
}

fn pedir_arreglo(arr : &mut[i32]) {
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&format!("Coloque el entero N°{}: ", i + 1));
    }
}

fn imprimir_arreglo(arr : &[i32]) {
    print!("\narr = [");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i + 1 < arr.len() {
            print!(", ");
        }
    }
    println!("]\n");
}

fn pedir_entero(texto : &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let x : i32 = input.trim().parse().expect("No se ingreso un entero");

    x
}