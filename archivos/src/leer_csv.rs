use std::{error::Error, path::Path};

pub fn leer_csv() {
    let archivo = Path::new("./src/archivo_prueba/prueba.csv");

    if let Err(e) = leer_archivo_csv(archivo.to_str().unwrap()) {
        eprintln!("{e}"); // eprintln! es una macro especial para pintar por terminal los errores
    }
}

fn leer_archivo_csv(archivo: &str) -> Result<(), Box<dyn Error>> {
    // Esta leyendo el archivo y colocando los datos en el struct Reader
    let mut reader: csv::Reader<std::fs::File> = csv::Reader::from_path(archivo)?;

    for res in reader.records() {
        let line = res?; // Como res es un Result<StringRecord, Error> utilizo el operador ?

        println!("{:?}", line);

        // El metodo get me da un Option
        if let Some(apellido) = line.get(1) {
            println!("El apellido es: {apellido}");
        }
    }

    Ok(())
}
