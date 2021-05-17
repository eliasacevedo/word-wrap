fn main() {
    let texto_prueba = "Este texto es una mera prueba para el ejercicio de hoy";
    println!("{}", principal(texto_prueba, 10));
}

fn principal(texto: &str, limite: usize) -> String {
    if limite <= 0 {
        return String::from("");
    }
    
    let mut resultado = String::from("");
    let mut texto_limitado = soporte(texto, limite as i32);

    while match texto_limitado.1 {
        Some(texto) => {
            let texto_mezclado = format!("\n{}", texto_limitado.0);
            resultado.push_str(&texto_mezclado);
            texto_limitado = soporte(texto, limite as i32);
            true
        }
        None => {
            let texto_mezclado = format!("\n{}", texto_limitado.0);
            resultado.push_str(&texto_mezclado);
            false
        }
    }{}

    return resultado;
}

fn soporte(texto: &str, limite: i32) -> (&str, Option<&str>) {
    if (texto.len() as i32) < limite {
        return (texto, None);
    }

    let resultado = ("", None);
    let mut pivote = limite - 1;
    loop {
        if pivote <= -1 {
            return (texto, None);
        }

        let letra = texto.chars().nth(pivote as usize);
        if letra == Some(' ') {
            return (
                &texto[..(pivote as usize)],
                Some(&texto[(pivote + 1) as usize ..])
            )
        }

        pivote -= 1;
    }

    return resultado;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn probar_longitud_limite() {
        let texto_prueba = "Este texto es una mera prueba para el ejercicio de hoy";
        let resultado = principal(texto_prueba, 0);
        assert!(resultado.len() == 0, "No debe retornar ningun texto")
    }
}