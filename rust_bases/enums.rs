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
    Disiembre,
}

enum Tiempo {
    Segundos,
    Minutos,
    Horas,
    Dia(Mes),
}

fn main() {
    let mi_tiempo = Tiempo::Dia(Mes::Julio);

    let segundos = timpo_a_segundos(mi_tiempo);

    println!("Son {segundos}s");
}

fn timpo_a_segundos(t: Tiempo) -> u32 {
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
