fn main() {
    let teste: String = String::from("500 + 32");
    let resultado = proximo(&teste);
    let mut resto = resultado.2;
    while resto != "o" {
        println!("({:?}, {:?})", resultado.0, resultado.1);
        let resultado = proximo(resto);
        resto = resultado.2;
    }
}

// Todo: mudar retorno para tipos corretos
// Todo: pensar numa lógica para manter o indice em chamadas diferentes 
fn proximo(entrada: &str) -> (usize, &str, &str)/* -> Result<(usize, &str, &str), Option<usize>> */ {
    for (indice, caractere) in entrada.char_indices() {
        if matches!(caractere, '+' | '-' | '/' | '*') {
            println!("operador '{caractere}' encontrado em {indice}");
            let operador_str = &entrada[indice..indice + caractere.len_utf8()];
            let resto_str = &entrada[indice + caractere.len_utf8()..];
            return (indice, operador_str, resto_str);
        }
        if let Some(_num_existe) = caractere.to_digit(10) {
            let mut num_string: String = String::new();
            for num_char in entrada[indice..].chars() {
                if let Some(_num_existe) = num_char.to_digit(10) {
                    num_string = num_string + &num_char.to_string()
                } else {
                    break
                }
            }
            // Todo: retornar (indice, caractere, resto da string)
            println!("numero '{num_string}' encontrado em {indice}")
        }
    }
    return (1, "o", "o");
}