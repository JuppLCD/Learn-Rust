fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6];

    // Puede dar error
    let tercero = &vector[2];
    println!("El tercer elemento es {tercero}");

    // No genera error, y retorna un ENUM del tipo Option
    match vector.get(2) {
        Some(tercero) => println!("El tercer elemento mediante match es {tercero}"),
        None => println!("No hay tercer elemento"),
    }

    // Recorrer un vector
    print!("\n");
    for elem in &vector {
        println!("Elemento: {elem}");
    }
}
