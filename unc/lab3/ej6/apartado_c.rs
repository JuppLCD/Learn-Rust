use std::io;

fn main(){
    println!("-- Asignacion de variables\n");
    
    let x: i32 = pedir_entero('x');
    let y: i32 = pedir_entero('y');
    let z: i32 = pedir_entero('z');
    let mut m: i32 = pedir_entero('m');

    imprime_estado(0, x, y, z, m);

    if x < y {
        println!("Modificacion por: x < y\n");
        m = x;
    }else {
        println!("Modificacion por: x >= y\n");
        m = y;
    }

    imprime_estado(1, x, y, z, m);

    if m < z {
        println!("Modificacion por: m < z\n");
    }else{
        println!("Modificacion por: m >= z\n");
        m = z;
    }

    imprime_estado(2, x, y, z, m);
}


fn pedir_entero(name: char) -> i32{
    let mut x = String::new();

    println!("Valor de {name}: ");
    io::stdin().read_line(&mut x).expect("Error al leer la entrada");

    let x: i32 = x.trim().parse().expect("No se pudo convertir a entero el input");

    x
}

fn imprime_entero(name: char, x : i32){
    println!("{name} = {x}");
}

fn imprime_estado(estado_numero : u32 ,x : i32, y : i32, z : i32, m : i32){
    println!("############### Estado σ{estado_numero} ###############");
    imprime_entero('x', x);
    imprime_entero('y', y);
    imprime_entero('z', z);
    imprime_entero('m', m);
}

/*
¿Qué ventajas encontras en esta nueva versión?.
El codigo se lee mas facil y se escribe menos.

¿Podrı́as escribir alguna otra función en ese ejercicio, cual?.
Podria crer una funcion que me imprema los estados de las variables en el programa. (imprimeEstado)

¿En qué otros ejercicios de ese Proyecto lo podrı́as utilizar?.
Las funciones pedirEntero e imprimeEntero las podria utilizar en casi todos los ejercicios.
 */