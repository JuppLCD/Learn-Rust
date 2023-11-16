use std::io;

fn main() {
    let mut input = String::new();
    let mut x;

    loop {
        println!("Ingrese un entero positivo mayor a cero: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la linea");

        match input.trim().parse() {
            Ok(num) => x = num,
            Err(_) => {
                println!("No ingresaste un numero");
                continue;
            }
        }

        if 0 <= x {
            break;
        }

        println!("No ingresaste un numero positivo mayor a cero");
    }

    let primo = nesimo_primo(x);

    println!("El primo N° {x} es: {primo}");
}

fn es_primo(n: i32) -> bool {
    let mut lo_es = true;
    let mut aux = 2;

    loop {
        if !(aux < n && lo_es) {
            break;
        }

        lo_es = n % aux != 0;
        aux += 1;
    }

    return lo_es;
}

fn nesimo_primo(n: i32) -> i32 {
    let mut contador = 0;

    let mut x = 2;

    loop {
        if !(contador <= n) {
            break;
        }

        if es_primo(x) {
            contador += 1;
        }

        x += 1;
    }

    x -= 1;

    return x;
}
