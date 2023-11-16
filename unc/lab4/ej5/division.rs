use std::io;
use std::assert;

struct DivT{
    cociente : i32,
    resto : i32
}


fn main(){
    let x = pedir_entero('x');
    let y = pedir_entero('y');

    if y == 0 {
        println!("Error: no se puede dividir por cero");
    }else {
        let d = division(x, y);
        println!("{x} / {y} = {y} * {} + {}", d.cociente, d.resto);
    }
}

fn pedir_entero(name : char) -> i32 {
    let mut input = String::new();

    println!("Asignar valor de {name}: ");
    io::stdin().read_line(&mut input).expect("No se pudo leer la linea");

    let x : i32 = input.trim().parse().expect("No se pudo pasar el input a entero");

    return x;
}

fn division(x : i32, y : i32) -> DivT{
    assert!(0 <= x && 0 < y);
    let resto = x % y;
    let cociente = (x - resto) / y;

    let d = DivT { resto, cociente };

    return d;
}