use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path};

#[derive(Serialize, Deserialize, Debug)]
struct Autor {
    nombre: String,
    // otras propiedades...
}

#[derive(Serialize, Deserialize, Debug)]
struct Articulo {
    autores: Vec<Autor>,
    titulo: String,
    texto: String,
}

impl Articulo {
    fn print(&self) {
        println!("Titulo del articulo: {}", self.titulo);
        println!(
            "Escrita por {} autores, los cuales son: ",
            self.autores.len()
        );

        for aut in &self.autores {
            println!("{}", aut.nombre)
        }
    }
}

pub fn leer_json() {
    let archivo = Path::new("./src/archivo_prueba/prueba.json");

    let json = read_to_string(archivo).expect("No se pudo abrir el archivo");

    let articulo: Articulo = serde_json::from_str(&json).unwrap();
    articulo.print();
}
