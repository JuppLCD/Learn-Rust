use std::io;

fn main() {
    let letra = pedir_caracter();

    let v = es_vocal(letra);

    println!("La letra ingresada {}, ¡¡{} UNA VOCAL!!", letra, if v {"ES"} else {"NO ES"});
}

fn es_vocal(letra : char) -> bool{
    return letra == 'a' || letra == 'e' || letra == 'i' || letra == 'o' || letra == 'u' || letra == 'A' || letra == 'E' || letra == 'I' || letra == 'O' || letra == 'U';
}

fn pedir_caracter() -> char {
    let mut input = String::new();

    println!("Ingrese un caracter: ");
    io::stdin().read_line(&mut input).expect("No se pudo leer la linea");

    if let Some(first_char) = input.trim().chars().next() {
        return first_char;
    } else {
        panic!("No se ingresó ningún caracter");
    }
}