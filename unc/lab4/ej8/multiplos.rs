use std::io;

fn main(){
    const N : usize = 5;

    let mut arr = [0; N];

    pedir_arreglo(&mut arr);

    let mut op: i32 ;
    loop {
        println!("\nSeleccione una de las siguientes opciones a realizar sobre 'arr':");
        println!("(1) Comprobar que todos los elementos son pares.");
        println!("(2) Comprobar si un número es múltiplo de algún elemento en el arreglo.");
        op = pedrir_entero("Opcion a seleccionar: ");

        if 1 <= op && op <= 2 {break};
    }

    match op {
        1 => {
            println!("\n############# Ha elegido la opción 1 #############");
            let son_pares = todos_pares(&arr);

            imprimir_arreglo(&arr);

            println!("{}TODOS SON PARES!!", if son_pares {""} else {"NO "});
        }

        2 => {
            println!("\n############# Ha elegido la opción 2 #############");
            let m = pedrir_entero("Ingrese el número que desea verificar si es múltiplo de algún elemento en el arreglo: ");

            let hay_multiplo = existe_multiplo(m, &arr);

            imprimir_arreglo(&arr);

            println!("{}HAY MULTIPLO DE {}!!", if hay_multiplo {""} else {"NO "} , m);
        }

        _ => {
            println!("\nOcurrio un error >:|\n");
        }
    }
}

fn pedir_arreglo(arr : &mut [i32] ) {
    println!("\n############# Coloque los valores para 'arr' #############");
    for i in 0..(arr.len()) {
        arr[i] = pedrir_entero(&(format!("Coloque el entero N°{}: ", i + 1)))
    }
}

fn pedrir_entero(texto : &str) -> i32 {
    let mut input = String::new();

    println!("{texto}");
    io::stdin().read_line(&mut input).expect("Error al leer la linea");

    let x : i32 = input.trim().parse().expect("Error: Se esperaba un entero");

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

fn todos_pares(arr : &[i32]) -> bool {
    let mut son_pares = true;

    for i in 0..(arr.len()) {
        if arr[i] % 2 != 0 {
            son_pares = false;
            return son_pares;
        }
    }

    son_pares
}

fn existe_multiplo(m : i32, arr : &[i32]) -> bool {
    let mut hay_multiplo = false;

    for i in 0..(arr.len()) {
        if arr[i] % m == 0 {
            hay_multiplo = true;
            return hay_multiplo;
        }
    }

    hay_multiplo
}