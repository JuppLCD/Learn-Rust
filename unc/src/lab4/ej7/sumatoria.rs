use std::io;

fn main() {
    const N : usize = 5;
    let mut arr = [0; N];

    pedir_arreglo(&mut arr);

    let sum = sumatoria(&arr);

    imprimir_arreglo(&arr);

    println!("La sumatoria de 'arr' es {sum}");
}

fn pedir_arreglo(arr : &mut[i32]) {
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&(format!("Coloque el entero N°{}: ", i + 1)));
    }
}

fn pedir_entero(texto : &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin().read_line(&mut input).expect("Error al leer la linea");

    let x : i32 = input.trim().parse().expect("No colocaste un entero");

    x
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

fn sumatoria(arr : &[i32]) -> i32 {
    let mut sum = 0;

    for num in arr{
        sum += num;
    }

        sum
}
