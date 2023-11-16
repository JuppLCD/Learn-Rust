use std::io;

fn main(){
    programa_a();
    println!("\n\n");
    programa_b();
    println!("\n\n");
    programa_c();
}

fn programa_a(){
    println!("~~~~~~~~~~~~~~~~~~ Programa a ~~~~~~~~~~~~~~~~~~\n\n");

    println!("############### Estado σ0 ###############\n");
    println!("-- Asignacion de variables\n");
    let mut x: i32 = scanf_propio('x');

    println!("Colocaste x: {x}");

    x = 5;

    println!("############### Estado σ1 ###############\n");
    println!("x: {x}");
}

fn programa_b(){
    println!("~~~~~~~~~~~~~~~~~~ Programa b ~~~~~~~~~~~~~~~~~~\n\n");


    println!("############### Estado σ0 ###############\n");
    println!("-- Asignacion de variables\n");
    let mut x: i32 = scanf_propio('x');
    let mut y: i32 = scanf_propio('y');


    x += y;

    println!("############### Estado σ1 ###############\n");
    println!("x: {x}");
    println!("y: {y}");

    y *= 2;

    println!("############### Estado σ2 ###############\n");
    println!("x: {x}");
    println!("y: {y}");
}

fn programa_c(){
    println!("~~~~~~~~~~~~~~~~~~ Programa c ~~~~~~~~~~~~~~~~~~\n\n");

    println!("############### Estado σ0 ###############\n");
    println!("-- Asignacion de variables\n");
    let mut x: i32 = scanf_propio('x');
    let mut y: i32 = scanf_propio('y');

    y *= 2;

    println!("############### Estado σ1 ###############\n");
    println!("x: {x}");
    println!("y: {y}");

    x += y;

    println!("############### Estado σ2 ###############\n");
    println!("x: {x}");
    println!("y: {y}");
}

fn scanf_propio(name : char) -> i32{
    let mut x = String::new();

    println!("Valor de {}: \n", name);
    io::stdin().read_line(&mut x).expect("Error al leer la entrada");

    let x: i32 = x.trim().parse().expect("No se pudo convertir a entero el input");

    x
}

/*
Ejecucion del ejercicio
-- 1.a
Ejecucion 1         |[σ0 : (x => 7)]|   =>   |[σ : (x => 5)]|ñ
Ejecucion 2         |[σ0 : (x => 15)]|  =>   |[σ : (x => 5)]|
Ejecucion 3         |[σ0 : (x => -86]|  =>   |[σ : (x => 5)]|

-- 1.b
Ejecucion 1         |[σ0 : (x => 7,   y => 3)]|   =>   |[σ : (x => 10,   y => 6)]|
Ejecucion 2         |[σ0 : (x => 15,  y => 7)]|   =>   |[σ : (x => 22,   y => 14)]|
Ejecucion 3         |[σ0 : (x => -86, y => 1)]|   =>   |[σ : (x => -85,  y => 2)]|

-- 1.c
Ejecucion 1         |[σ0 : (x => 7,   y => 3)]|   =>   |[σ : (x => 13,   y => 6)]|
Ejecucion 2         |[σ0 : (x => 15,  y => 7)]|   =>   |[σ : (x => 29,   y => 14)]|
Ejecucion 3         |[σ0 : (x => -86, y => 1)]|   =>   |[σ : (x => -84,  y => 2)]|
*/