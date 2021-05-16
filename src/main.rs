fn main() {
    println!("Hello, world!");
}

fn principal(texto: &str, limite: usize) -> String {
    if (limite <= 0) {
        return String::from("");
    }
    
    let mut resultado = String::from("");
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