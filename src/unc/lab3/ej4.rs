use std::io;

fn main(){
    apartado_a();
    println!("\n\n");
    apartado_b();
}

fn apartado_a(){
    println!("~~~~~~~~~~~~~~~~~~ Apartado a ~~~~~~~~~~~~~~~~~~");
    println!("-- Asignacion de variables");
    let mut x: i32 = scanf_propio('x');
    let y: i32 = scanf_propio('y');

    pintar_estado(0, vec![('x', x), ('y', y)]);


    if x >= y {
        println!("Modificacion por: x >= y");
        x = 0;
    }else{
        println!("Modificacion por: x < y");
        x = 2;
    }

    pintar_estado(1, vec![('x', x), ('y', y)]);
}

fn apartado_b(){
    println!("~~~~~~~~~~~~~~~~~~ Apartado b ~~~~~~~~~~~~~~~~~~");
    println!("-- Asignacion de variables");
    let x: i32 = scanf_propio('x');
    let y: i32 = scanf_propio('y');
    let z: i32 = scanf_propio('z');
    let mut m: i32 = scanf_propio('m');

    pintar_estado(0, vec![('x', x), ('y', y), ('z', z), ('m', m)]);



    if x < y {
        println!("Modificacion por: x < y");
        m = x;
    }else {
        println!("Modificacion por: x >= y");
        m = y;
    }

    pintar_estado(1, vec![('x', x), ('y', y), ('z', z), ('m', m)]);



    if m < z {
        println!("Modificacion por: m < z");
    }else{
        println!("Modificacion por: m >= z");
        m = z;
    }

    pintar_estado(2, vec![('x', x), ('y', y), ('z', z), ('m', m)]);
}

fn scanf_propio(name : char) -> i32{
    let mut x = String::new();

    println!("Valor de {}:", name);
    io::stdin().read_line(&mut x).expect("Error al leer la entrada");

    let x: i32 = x.trim().parse().expect("No se pudo convertir a entero el input");

    x
}

fn pintar_estado(n : u32, variables : Vec<(char, i32)>){
    println!("############### Estado σ{n} ###############");

    for (name, valor) in variables {
        println!("{name}: {valor}");
    }
}


/*
Ejecucion del ejercicio
-- a
Ejecucion 1         |[σ0 : (x => 1,   y => 2)]|   =>   |[σ : (x => 2,   y => 2)]|
Ejecucion 2         |[σ0 : (x => 15,  y => 7)]|   =>   |[σ : (x => 0,   y => 7)]|

-- b
Ejecucion 1         |[σ0 : (x => 5,   y => 4,  z => 8,  m => 0)]|   =>   |[σ0 : (x => 5,   y => 4,  z => 8,  m => 4)]|
Ejecucion 2         |[σ0 : (x => 1,   y => 2,  z => 3,  m => 4)]|   =>   |[σ0 : (x => 1,   y => 2,  z => 3,  m => 1)]|
Ejecucion 3         |[σ0 : (x => 4,   y => 3,  z => 2,  m => 1)]|   =>   |[σ0 : (x => 4,   y => 3,  z => 2,  m => 2)]|
*/