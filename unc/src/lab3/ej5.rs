use std::io;

fn main(){
    apartado_a();
    apartado_b();
}

fn apartado_a(){
    println!("-------------------- Apartado A --------------------");
    ejercicio_h();

    ejercicio_i();
}

fn ejercicio_h(){
    println!("-------------------- Ejercicio 1.h --------------------");
    println!("-- Asignacion de variables");
    let mut i  = scanf_propio('i');

    pintar_estado(0, vec![('i', i)]);

    while i != 0{
        println!("Valor de i previo a la resta: {i}");
        i -= 1;
        println!("Valor de i posterior a la resta: {i}");
    }

    println!("Termina el programa con i = {i}");
}

fn ejercicio_i(){
    println!("-------------------- Ejercicio 1.i --------------------");

    println!("-- Asignacion de variables");
    let mut i  = scanf_propio('i');

    pintar_estado(0, vec![('i', i)]);

    while i != 0    {
        println!("Valor de i previo a la resta: {i}");
        i = 0;
        println!("Valor de i posterior a la resta: {i}");
    }

    println!("Termina el programa con i = {i}");
}

fn apartado_b(){
    println!("-------------------- Apartado B --------------------");
    punto_1();
    println!("========= Punto 2 =========");
    punto_2();
}

fn punto_1(){
    println!("========= Punto 1 =========");

    println!("-- Asignacion de variables");
    let mut x  = scanf_propio('x');
    let y  = scanf_propio('y');
    let mut i  = scanf_propio('i');

    pintar_estado(0, vec![('x', x),('y', y),('i', i)]);

    i = 0;
    while x >= y {
        println!("Iteracion Nª {}", i + 1);
        println!("Previo a las operaciones: x = {}, y = {}, i = {}", x, y, i);
        x -= y;
        i += 1;
        println!("Posteriores a las operaciones: x = {}, y = {}, i = {}", x, y, i);
    }
}

fn punto_2(){
    println!("-- Asignacion de variables");
    let x  = scanf_propio('x');
    let mut i  = scanf_propio('i');

    let temp  = scanf_propio('r');
    let mut res = temp == 0 ;

    pintar_estado(0, vec![('x', x),('i', i),('r', if res {1} else {0})]);

    i = 2;

    while i < x && res {
        println!("Iteracion Nª {}", i - 1);
        println!("Previo a las operaciones: x = {}, res = {}, i = {}", x, res , i);

        res = res && (x % i != 0);
        i += 1;

        println!("Posteriores a las operaciones: x = {}, res = {}, i = {}", x, res , i);
    }
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
Ejecucion h    |[σ0 : (i => 4)]|     =>   |[σ : (i => 0)]|
Ejecucion i    |[σ0 : (i => 400)]|   =>   |[σ : (i => 0)]|

-- b
Ejecucion 1    |[σ0 : (x => 13,   y => 3,  i => 0)]|        =>   |[σ : (x => 1,   y => 3,  i => 4)]|
Ejecucion 2    |[σ0 : (x => 5,   res => False,  i => 0)]|   =>   |[σ : (x => 5,   res => True,  i => 5)]|
*/