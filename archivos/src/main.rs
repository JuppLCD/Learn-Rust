// Coloco el pub para que no me tire advertencias el compilador
pub mod deserealizar_json;
pub mod leer_archivo;
pub mod leer_csv;
pub mod serializando_json;

fn main() {
    leer_archivo::leer_archivo();
    leer_csv::leer_csv();
    // println!("Hola mundo");
}
