use std::io;

struct TDatos {
    maximo: f32,
    minimo: f32,
    promedio: f32,
}

fn main() {
    const N: usize = 5;
    let mut arr: [f32; N] = [0.0; N];

    pedir_arreglo(&mut arr);
    imprimir_arreglo(&arr);

    let dato = stats(&arr);

    println!(
        "\nPromedio: {:.4}\nMinimo: {:.4}\nMaximo: {:.4}",
        dato.promedio, dato.minimo, dato.maximo,
    );
}

fn pedir_entero(texto: &str) -> f32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let x: f32 = input.trim().parse().expect("No se ingreso un entero");

    x
}

fn pedir_arreglo(arr: &mut [f32]) {
    for i in 0..(arr.len()) {
        arr[i] = pedir_entero(&format!("Coloque el entero N°{}: ", i + 1));
    }
}

fn imprimir_arreglo(arr: &[f32]) {
    print!("\narr = [");
    for i in 0..arr.len() {
        print!("{}", arr[i]);

        if i + 1 < arr.len() {
            print!(", ");
        }
    }
    println!("]\n");
}

fn stats(arr: &[f32]) -> TDatos {
    let mut datos = TDatos {
        maximo: std::f32::MIN,
        minimo: std::f32::MAX,
        promedio: 0.0,
    };

    let mut sum: f32 = 0.0;

    for elem in arr {
        sum += elem;
        if *elem < datos.minimo {
            datos.minimo = *elem;
        } else if datos.maximo < *elem {
            datos.maximo = *elem;
        }
    }

    datos.promedio = sum / arr.len() as f32;

    return datos;
}
