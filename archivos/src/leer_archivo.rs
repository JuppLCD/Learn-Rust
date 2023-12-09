/* use std::{
    fs::File,
    io::{ErrorKind, Read},
}; */

use std::fs::read_to_string;

pub fn leer_archivo() {
    let archivo = "prueba.txt";
    //* Forma larga de leer un archivo, con validacion de que existe y en caso de que no su creacion
    /* let file = File::open(archivo);

    let mut my_file = match file {
        Ok(my_file) => my_file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(archivo) {
                Ok(my_file) => my_file,
                Err(_) => panic!("Ocurrio un error al crear el archivo {archivo}"),
            },
            _ => panic!("Error desconocido"),
        },
    };

    let mut text_file = String::new();
    match my_file.read_to_string(&mut text_file) {
        Ok(_) => println!("{text_file}"),
        Err(e) => println!("Error: {e}"),
    }; */

    //* Utilizando la funcion read_to_string para leer archivos de forma rapida
    for line in read_to_string(archivo)
        .expect("No se pudo abrir el archivo")
        .lines()
    {
        println!("{line}");
    }
}
