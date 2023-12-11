use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Autor {
    nombre: String,
    // otras propiedades...
}

impl Autor {
    fn new(nombre: &str) -> Self {
        Autor {
            nombre: nombre.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Articulo {
    autores: Vec<Autor>,
    titulo: String,
    texto: String,
}

pub fn creando_json() {
    let autor_1 = Autor::new("Claudia Pallares");
    let autor_2 = Autor::new("Laila Arnau");

    let titulo_articulo =
        String::from("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet");
    let texto_articulo = String::from("Maecenas arcu est, rhoncus a orci placerat, elementum dignissim mi. Vestibulum tincidunt, nulla eget volutpat gravida, mauris nunc egestas arcu, ullamcorper congue lacus ipsum nec odio.");

    let articulo_prueba = Articulo {
        titulo: titulo_articulo,
        texto: texto_articulo,
        autores: vec![autor_1, autor_2],
    };

    let json = serde_json::to_string(&articulo_prueba).unwrap();

    println!("JSON : {json}");
}
