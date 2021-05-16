fn main() {
    println!("Hello, world!");
}

fn envoltorio(texto: &str, columna: usize) -> String {
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

fn funcion_soporte(texto: &str, columna: usize) -> (&str, Option<&str>) {
    let mut resultado: (&str, Option<&str>) = ("", None);

    return resultado;
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