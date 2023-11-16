use std::io;

fn main() {
    const N: usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);

    imprimir_arreglo(&arr);

    let min_par = minimo_pares(&arr);
    let min_impar = minimo_impares(&arr);

    println!("El minimo par del arreglo es: {min_par}");
    println!("El minimo impar del arreglo es: {min_impar}");

    println!(
        "El elemento minimo del array es: {}",
        if min_par < min_impar {
            min_par
        } else {
            min_impar
        }
    );
}

fn pedir_entero(texto: &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let x: i32 = input.trim().parse().expect("No se ingreso un entero");

    x
}

fn pedir_arreglo(arr: &mut [i32]) {
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&format!("Coloque el entero N°{}: ", i + 1));
    }
}

fn imprimir_arreglo(arr: &[i32]) {
    print!("\narr = [");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i + 1 < arr.len() {
            print!(", ");
        }
    }
    println!("]\n");
}

fn minimo_pares(arr: &[i32]) -> i32 {
    let mut min_par = i32::MAX;

    for i in 0..(arr.len()) {
        if arr[i] % 2 == 0 && arr[i] < min_par {
            min_par = arr[i];
        }
    }

    min_par
}

fn minimo_impares(arr: &[i32]) -> i32 {
    let mut min_impar = i32::MAX;

    for i in 0..(arr.len()) {
        if arr[i] % 2 == 1 && arr[i] < min_impar {
            min_impar = arr[i];
        }
    }

    min_impar
}
