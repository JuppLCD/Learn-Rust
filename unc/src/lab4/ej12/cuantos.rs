use std::io;

struct TComp {
    menores: i32,
    iguales: i32,
    mayores: i32,
}

fn main() {
    const N: usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);
    imprimir_arreglo(&arr);

    let elem =
        pedir_entero("Coloque el numero entero a comparar con los elemntos del array dado: ");

    let cuant = cuantos(&arr, elem);

    println!(
        "\nN° Menores: {}\nN° Iguales: {}\nN° Mayores: {}",
        cuant.menores, cuant.iguales, cuant.mayores
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

fn cuantos(arr: &[i32], elem: i32) -> TComp {
    let mut cuant = TComp {
        menores: 0,
        iguales: 0,
        mayores: 0,
    };

    for elemet_in_arr in arr {
        if elemet_in_arr < &elem {
            cuant.menores += 1;
        } else if &elem == elemet_in_arr {
            cuant.iguales += 1;
        } else {
            cuant.mayores += 1;
        }
    }

    cuant
}
use std::io;

struct TComp {
    menores: i32,
    iguales: i32,
    mayores: i32,
}

fn main() {
    const N: usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);
    imprimir_arreglo(&arr);

    let elem =
        pedir_entero("Coloque el numero entero a comparar con los elemntos del array dado: ");

    let cuant = cuantos(&arr, elem);

    println!(
        "\nN° Menores: {}\nN° Iguales: {}\nN° Mayores: {}",
        cuant.menores, cuant.iguales, cuant.mayores
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

fn cuantos(arr: &[i32], elem: i32) -> TComp {
    let mut cuant = TComp {
        menores: 0,
        iguales: 0,
        mayores: 0,
    };

    for elemet_in_arr in arr {
        if elemet_in_arr < &elem {
            cuant.menores += 1;
        } else if &elem == elemet_in_arr {
            cuant.iguales += 1;
        } else {
            cuant.mayores += 1;
        }
    }

    cuant
}
