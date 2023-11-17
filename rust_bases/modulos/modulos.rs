mod modulo_1 {
    fn privada_md_1() {}
    pub mod modulo_2 {
        fn privada_md_2() {}

        pub fn publica_md_2() {
            println!("publica_md_2");
        }
    }
}

mod modulo_3;

fn main() {
    // Debe tirar error, ya que todo dentro de los modulos son privados por defecto
    // modulo_1::privada_md_1();
    // modulo_1::modulo_2::privada_md_2();

    // Se importa correctamente
    // Ruta absoluta
    crate::modulo_1::modulo_2::publica_md_2();

    // Ruta relativa
    modulo_1::modulo_2::publica_md_2();
    self::modulo_1::modulo_2::publica_md_2();

    // Archivos externos
    modulo_3::publica_md_3();
    modulo_3::saludo_mf_3();
}
