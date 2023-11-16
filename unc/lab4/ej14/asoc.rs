use std::io;

type TClave = char;
type TValor = i32;

struct TAsoc {
    clave: TClave,
    valor: TValor,
}

impl Default for TAsoc {
    fn default() -> Self {
        TAsoc {
            clave: ' ',
            valor: 0,
        }
    }
}

fn main() {
    const N: usize = 5;
    let mut arr: [TAsoc; N] = Default::default();

    pedir_arreglo_asoc(&mut arr);
    imprimir_arreglo_asoc(&arr);

    let c = pedir_caracter("Ingrese la clave a buscar: ");

    let existe = asoc_existe(&arr, c);

    println!(
        "LA CLAVE {}EXISTE EN EL ARREGLO",
        if existe { "" } else { "NO " }
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

fn pedir_caracter(texto: &str) -> char {
    let mut input = String::new();

    println!("{texto}");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let letra: char = input
        .trim()
        .chars()
        .next()
        .expect("No se ingreso un caracter");

    letra
}

fn pedir_arreglo_asoc(arr: &mut [TAsoc]) {
    println!("\n############# Coloque los valores para 'arr asociativo' #############");

    for i in 0..(arr.len()) {
        println!("-- Elemnto N°{} --", i + 1);

        let asoc = TAsoc {
            clave: pedir_caracter("Ingrese la clave: "),
            valor: pedir_entero("Ingrese el valor: "),
        };

        arr[i] = asoc;
    }
}

fn imprimir_arreglo_asoc(arr: &[TAsoc]) {
    print!("\narr = [");
    for i in 0..arr.len() {
        print!("{{ {} :", arr[i].clave);
        print!(" {} }}", arr[i].valor);

        if i + 1 < arr.len() {
            print!(", ");
        }
    }
    println!("]\n");
}

fn asoc_existe(arr: &[TAsoc], c: char) -> bool {
    for elem in arr {
        if elem.clave.cmp(&c).is_eq() {
            return true;
        }
    }

    false
}
