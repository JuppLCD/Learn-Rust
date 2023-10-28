use std::io;

fn main() {
    const N : usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);
    imprimir_arreglo(&arr);

    let primeros_iguales = prim_iguales(&arr);
    println!("Primeros iguales: {primeros_iguales}");
    imprimir_arreglo_hasta(&arr, primeros_iguales + 1);
}

fn pedir_entero(texto: &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let x : i32 = input.trim().parse().expect("No se ingreso un entero");

    x
}

fn pedir_arreglo(arr: &mut[i32]) {
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&format!("Coloque el entero N°{}: ", i + 1));
    }
}

fn imprimir_arreglo_hasta(arr: &[i32], hasta : usize) {
    print!("\narr = [");
    for i in 0..hasta{
        print!("{}", arr[i]);
        if i + 1 < hasta {
            print!(", ");
        }
    }
    println!("]\n");
}

fn imprimir_arreglo(arr: &[i32]) {
    imprimir_arreglo_hasta(arr, arr.len());
}

fn prim_iguales(arr: &[i32]) -> usize {
    let mut x : usize = 0;

    for i in 0..(arr.len() - 1) {
        if arr[i] == arr[i + 1]{
            x += 1;
        }
        else {
            break;
        }
    }

    return x;
}
