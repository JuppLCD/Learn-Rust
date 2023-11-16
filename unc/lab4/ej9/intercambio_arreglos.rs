use std::io;

fn main() {
    const N : usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);

    imprimir_arreglo(&arr);

    println!("Ingrese las posiciones i, j en los cuales los elementos de van a intercambiar:");
    let i = pedir_entero("Ingresar el valor para i: ");
    let j = pedir_entero("Ingresar el valor para j: ");

    if 0 <= i && i < (N as i32) && 0 <= j && j < (N as i32) {
        intercambiar(&mut arr, i as usize, j as usize);

        imprimir_arreglo(&arr);
    }
    else {
        println!("Error: Los valores i y/o j no se encuentran dentro del rango 0 <= i,j < {N}. (i={i}, j={j})");
    }
}

fn pedir_entero(texto : &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin().read_line(&mut input).expect("Error al leer la linea");

    let x : i32 = input.trim().parse().expect("Error se esperaba un entero");

    x
}

fn pedir_arreglo(arr : &mut[i32]) {
    println!("\n############# Coloque los valores para 'arr' #############");
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&format!("Coloque el entero N°{}: ", i + 1));
    }
}

fn imprimir_arreglo(arr : &[i32]) {
    print!("\narr = [");
    for i in 0..(arr.len()) {
        print!("{}", arr[i]);

        if i + 1 < arr.len() {
            print!(", ");
        }
    }
    println!("]\n");
}

fn intercambiar(arr : &mut[i32], i : usize, j : usize) {
    let aux = arr[i];

    arr[i] = arr[j];
    arr[j] = aux;
}
