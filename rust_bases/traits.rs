// traits, similar a interfaces
trait Vehiculo {
    fn encender(&self);
    fn detener(&self);
    fn bocina(&self);

    // Algo asi como polimorfismo en funciones asociadas
    fn chequear_vehiculo_dentro(vehiculo: &Self) {
        vehiculo.encender();
        vehiculo.bocina();
        vehiculo.detener();
    }
}

struct Auto {
    modelo: String,
}

struct Moto {
    marca: String,
}

// Implementacion de un trait (interfaz)
impl Vehiculo for Auto {
    fn encender(&self) {
        println!("El auto {} encendio.", self.modelo);
    }

    fn detener(&self) {
        println!("El auto {} se apago.", self.modelo);
    }

    fn bocina(&self) {
        println!("La bocina del auto {}, hace piiipi!", self.modelo);
    }
}

impl Vehiculo for Moto {
    fn encender(&self) {
        println!("La moto {} encendio.", self.marca);
    }

    fn detener(&self) {
        println!("La moto {} se apago.", self.marca);
    }

    fn bocina(&self) {
        println!("La moto {} hace peeepeeep!", self.marca);
    }
}

// Polimorfismo en funcion
fn chequear_vehiculo(vehiculo: &impl Vehiculo) {
    vehiculo.encender();
    vehiculo.bocina();
    vehiculo.detener();
}

fn main() {
    let auto1 = Auto {
        modelo: String::from("SUV"),
    };
    let moto1 = Moto {
        marca: String::from("Sportbike"),
    };

    chequear_vehiculo(&auto1);

    print!("\n");

    chequear_vehiculo(&moto1);

    print!("\n");

    Auto::chequear_vehiculo_dentro(&auto1);
}
