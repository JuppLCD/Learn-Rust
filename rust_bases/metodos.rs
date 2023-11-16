struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn nuevo(ancho: u32, alto: u32) -> Rectangulo {
        Rectangulo { ancho, alto }
    }

    fn clonar(otro: &Rectangulo) -> Rectangulo {
        Rectangulo { ..(*otro) }
    }
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        otro.alto <= self.alto && otro.ancho <= self.ancho
    }
}

fn main() {
    let rec1 = Rectangulo {
        ancho: 30,
        alto: 50,
    };

    let area_rec1 = rec1.area();

    println!("El area del rectagulo es: {area_rec1}");

    let rec2 = Rectangulo {
        ancho: 23,
        alto: 50,
    };

    let rec3 = Rectangulo {
        ancho: 80,
        alto: 15,
    };

    println!("Rec1 puede contener a rec2? {}", rec1.puede_contener(&rec2));
    println!("Rec1 puede contener a rec3? {}", rec1.puede_contener(&rec3));

    let rec4 = Rectangulo::nuevo(100, 150);

    let rec5 = Rectangulo::clonar(&rec4);

    println!("Rec5 puede contener a rec3? {}", rec5.puede_contener(&rec3));
}
