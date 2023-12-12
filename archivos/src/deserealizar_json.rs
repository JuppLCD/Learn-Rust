use serde::{Deserialize, Serialize};

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
    let json = r#"
        {
            "titulo" : "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...",
            "texto" : "Maecenas arcu est, rhoncus a orci placerat, elementum dignissim mi. Vestibulum tincidunt, nulla eget volutpat gravida, mauris nunc egestas arcu, ullamcorper congue lacus ipsum nec odio. Fusce a arcu placerat, consequat turpis eu, tempor dolor. Nulla facilisi. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Mauris dignissim rutrum pulvinar.",
            "autores": [
                {
                    "nombre" : "Miriam Mas"
                },
                {
                    "nombre" : "Hugo Castellanos"
                },
                {
                    "nombre" : "Dario Colomer"
                }
            ]
        }
    "#;

    let articulo: Articulo = serde_json::from_str(json).unwrap();
    articulo.print();
}
