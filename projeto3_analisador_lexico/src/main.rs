fn main() {
    let testes = [
        "450 + 20", "450     +     20", "450+20", "0+-0", "0 +++",
        "10+a", "10 + 20a", "30 🐧 45", "78 + 3 🐧 23", "+2🐧-*", "40 🦀🦀+ 3",
        "🦀🦀 564 / 444", "554a🦀 + 34🐧", "34 +** 34"
    ];
    for teste in testes{
        let mut resultado = proximo(&teste);
        let mut pos_absoluta: usize = 1;

        println!("{teste}");
        while let Ok((pos_no_slice, conteudo, resto)) = resultado {
            pos_absoluta += pos_no_slice;
            print!("(\"{conteudo}\", {pos_absoluta}) ");
            pos_absoluta += conteudo.len();

            resultado = proximo(resto)
        }
        if let Err(Some(pos_erro)) = resultado {
            pos_absoluta += pos_erro;
            print!("Erro na posição {pos_absoluta}")
        }
        print!("\n")
    }
}

fn proximo(entrada: &str) -> Result<(usize, &str, &str), Option<usize>> {
    for (indice, caractere) in entrada.char_indices() {
        if caractere.is_whitespace() || caractere == '🦀'{
            continue
        }
        if matches!(caractere, '+' | '-' | '/' | '*' | '🐧') {
            let operador_str = &entrada[indice..indice + caractere.len_utf8()];
            let resto = &entrada[indice + caractere.len_utf8()..];
            return Ok((indice, operador_str, resto))
        }
        if caractere.is_digit(10) {
            let indice_inicio_num = indice;
            let mut indice_fim_num = indice;
            for (offset_no_slice, num_char) in entrada[indice..].char_indices() {
                if num_char.is_digit(10) {
                    indice_fim_num = indice + offset_no_slice + 1;
                } else {
                    break;
                }
            }
            
            let num_encontrado = &entrada[indice_inicio_num..indice_fim_num];
            let resto = &entrada[indice_fim_num..];
            return Ok((indice, num_encontrado, resto))
        }
        return Err(Some(indice))
    }
    return Err(None)
}