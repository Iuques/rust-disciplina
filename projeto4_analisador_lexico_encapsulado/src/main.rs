fn main() {
    let testes = [
        "450 + 20", "450     +     20", "450+20", "0+-0", "0 +++",
        "10+a", "10 + 20a", "30 🐧 45", "78 + 3 🐧 23", "+2🐧-*", "40 🦀🦀+ 3",
        "🦀🦀 564 / 444", "554a🦀 + 34🐧", "34 +** 34"
    ];
    
    for teste in testes {
        let mut analisador = Analisador::novo(teste);

        println!("{teste}");
        while let Ok((pos, conteudo)) = analisador.proximo() {
            print!("(\"{conteudo}\", {pos}) ");
        }
        if let Err(Some(pos_erro)) = analisador.proximo() {
            print!("Erro na posição {pos_erro}");
        }
        print!("\n");
    }
}

struct Analisador<'a> {
    pos: usize,
    prox: &'a str
}

impl<'a> Analisador<'a> {
    fn novo(entrada: &'a str) -> Self {
        Analisador { 
            pos: 1, 
            prox: entrada 
        }
    }
    
    fn proximo(&mut self) -> Result<(usize, &str), Option<usize>> {
        for (indice, caractere) in self.prox.char_indices() {
            if caractere.is_whitespace() || caractere == '🦀'{
                continue
            }
            if matches!(caractere, '+' | '-' | '/' | '*' | '🐧') {
                let operador_str: &str= &self.prox[indice..indice + caractere.len_utf8()];
                let resto: &str = &self.prox[indice + caractere.len_utf8()..];

                let pos_absoluta = self.pos + indice;
                self.prox = resto;
                self.pos = pos_absoluta + operador_str.len();

                return Ok((pos_absoluta, operador_str))
            }
            if caractere.is_digit(10) {
                let indice_inicio_num = indice;
                let mut indice_fim_num = indice;
                for (offset_no_slice, num_char) in self.prox[indice..].char_indices() {
                    if num_char.is_digit(10) {
                        indice_fim_num = indice + offset_no_slice + 1;
                    } else {
                        break
                    }
                }

                let num_encontrado = &self.prox[indice_inicio_num..indice_fim_num];
                let resto = &self.prox[indice_fim_num..];

                let pos_absoluta = self.pos + indice_inicio_num;
                self.prox = resto;
                self.pos = pos_absoluta + num_encontrado.len();

                return Ok((pos_absoluta, num_encontrado))
            }

            let pos_absoluta = self.pos + indice;
            return Err(Some(pos_absoluta))
        }
        return Err(None)
    }
}