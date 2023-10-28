struct TPersona {
    nombre: String,
    edad: u8,
    altura: f32,
    peso: f32,
}

fn main() {
    let p1 = TPersona {
        nombre: "Paola".to_string(),
        edad: 21,
        altura: 1.85,
        peso: 75.0,
    };
    let p2 = TPersona {
        nombre: "Luis".to_string(),
        edad: 54,
        altura: 1.75,
        peso: 69.0,
    };
    let p3 = TPersona {
        nombre: "Julio".to_string(),
        edad: 40,
        altura: 1.70,
        peso: 80.0,
    };

    let arr = [p1, p2, p3];

    println!("El peso promedio es {:.4}", peso_promedio(&arr));

    let p = persona_de_mayor_edad(&arr);
    println!("El nombre de la persona con mayor edad es {}", p.nombre);

    let p = persona_de_menor_altura(&arr);
    println!("El nombre de la persona con menor altura es {}", p.nombre);
}

fn peso_promedio(arr: &[TPersona]) -> f32 {
    let mut peso_total: f32 = 0.0;

    for persona in arr {
        peso_total += persona.peso;
    }

    peso_total / (arr.len() as f32)
}

fn persona_de_mayor_edad(arr: &[TPersona]) -> &TPersona {
    let mut persona_mayor_edad = &arr[0];

    for persona in arr {
        if persona_mayor_edad.edad < persona.edad {
            persona_mayor_edad = persona;
        }
    }

    persona_mayor_edad
}

fn persona_de_menor_altura(arr: &[TPersona]) -> &TPersona {
    let mut persona_menor_altura = &arr[0];

    for persona in arr {
        if persona.altura < persona_menor_altura.altura {
            persona_menor_altura = persona;
        }
    }

    persona_menor_altura
}
