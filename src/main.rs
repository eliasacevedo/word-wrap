fn main() {
    println!("Hello, world!");
}

fn principal(texto: &str, limite: usize) -> String {
    if limite <= 0 {
        return String::from("");
    }
    
    let mut resultado = String::from("");
    let mut texto_limitado = soporte(texto, limite as i32);

    while match texto_limitado.1 {
        Some(texto) => {
            resultado.push_str(format!("\n{}", texto_limitado.0));
            texto_limitado = soporte(texto, limite as i32);
            true
        }
        None => {
            resultado.push_str(texto_limitado.0);
            false
        }
    }{}

    return resultado;
}

fn soporte(texto: &str, limite: i32) -> (&str, Option<&str>) {
    let mut resultado = ("", None);

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