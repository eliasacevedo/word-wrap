fn main() {
    println!("{}", envoltorio("Este es un texto de ejemplo que sera utilizado en la segunda grabacion del Kata", 1));
}

fn envoltorio(texto: &str, columna: i32) -> String {
    if columna == 0 {
        return String::from("");
    }
    let mut texto_apilado = String::from("");
    let mut resultado = funcion_soporte(texto, columna);

    while match resultado.1 {
        Some(texto_parcial) => {
            let palabra_soporte = format!("\n{}", resultado.0);
            texto_apilado.push_str(&palabra_soporte);
            resultado = funcion_soporte(&texto_parcial, columna);
            true
        }
        None => {
            let palabra_soporte = format!("\n{}", resultado.0);
            texto_apilado.push_str(&palabra_soporte);
            false
        }
    }{} 

    return texto_apilado;
}

fn funcion_soporte(texto: &str, columna: i32) -> (&str, Option<&str>) {
    if (texto.len() as i32) < columna {
        return (texto, None);
    }

    let mut indice_actual = columna - 1;
    loop {
        if indice_actual == -1{
            return (texto, None);
        }

        let letra = texto.chars().nth(indice_actual as usize);
        if letra == Some(' ') {
            return (
                &texto[..(indice_actual as usize)],
                Some(&texto[(indice_actual as usize) + 1 ..])
            )
        }

        indice_actual -= 1;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_longitud_columna() {
        let texto = "Este es un texto de ejemplo que sera utilizado en la segunda grabacion del Kata";
        let resultado = envoltorio(texto, 0);

        assert!(resultado.len() <= 0, "No debe retornar nada");
    }
}