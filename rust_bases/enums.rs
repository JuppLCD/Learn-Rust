enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre,
}

enum Tiempo {
    Segundos,
    Minutos,
    Horas,
    Dia(Mes),
}

fn main() {
    let mi_tiempo = Tiempo::Dia(Mes::Julio);
    let mi_tiempo_2 = Tiempo::Minutos;

    let segundos = timpo_a_segundos(&mi_tiempo);
    let contador = prueba_if_let(&mi_tiempo_2);

    println!("Son {segundos}s");

    println!("{contador}");
}

fn timpo_a_segundos(t: &Tiempo) -> u32 {
    match t {
        Tiempo::Segundos => 1,
        Tiempo::Minutos => 60,
        Tiempo::Horas => 60 * 60,
        Tiempo::Dia(mes) => match mes {
            Mes::Julio => 60 * 60 * 24 * 10,
            _ => 60 * 60 * 24,
        },
    }
}

fn prueba_if_let(t: &Tiempo) -> u32 {
    let mut contador = 0_u32;

    if let Tiempo::Minutos = t {
        contador += 1;
    } else {
        println!("Tiempo distinto de minutos");
    }

    contador
}
