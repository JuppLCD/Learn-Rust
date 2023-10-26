fn main(){
    // Variable auxiliar para x,y,z
    let a = 1; // Cumple para cualquier valor de a

    // Variables
    let x = 4*a;
    let y = -x;
    let z = 8*a;

    let b = false;
    let w = false;
    /*
    -- Otras posibles convinaciones
    b = true;
    w = false;

    b = true;
    w = true;
    */

    // Expresiones
    let e1 = x % 4 == 0;
    let e2 = x + y == 0 && y - x == (-1) * z;
    let e3 = !b && w;


    println!("El resultado de la expresion N°1 es: {e1}");
    println!("El resultado de la expresion N°2 es: {e2}");
    println!("El resultado de la expresion N°3 es: {e3}");
}