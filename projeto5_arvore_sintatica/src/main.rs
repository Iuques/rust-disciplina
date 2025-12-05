use Expressao::*;

fn main() {
    let testes = get_testes();

    for teste in testes {
        teste.imprimir();
        print!("\n");
        teste.imprimir_arvore();
        println!("Resultado: {:?}", teste.avaliar());
        print!("\n");
    }

}

enum Expressao {
    Operacao(char, Box<Expressao>, Box<Expressao>), // char: operador, expressões da esquerda e direita nessa ordem 
    Negacao(Box<Expressao>), // negação de uma expressão
    Numero(i64) // valor numérico
}

impl Expressao {
    fn avaliar(&self) -> Option<i64> {
        match self {
            Operacao(operador, exp1, exp2) => {
                let resultado_exp1 = exp1.avaliar()?;
                let resultado_exp2 = exp2.avaliar()?;

                match operador {
                    '+' => {return Some(resultado_exp1 + resultado_exp2)}
                    '-' => {return Some(resultado_exp1 - resultado_exp2)}
                    '*' => {return Some(resultado_exp1 * resultado_exp2)}
                    '/' => {
                        if resultado_exp2 != 0 {
                            return Some(resultado_exp1 / resultado_exp2)
                        } else {
                            return None
                        }
                    }
                    '%' => {
                        if resultado_exp2 != 0 {
                            return Some(resultado_exp1 % resultado_exp2)
                        } else {
                            return None
                        }
                    }
                    _ => {return None}
                }
           }
           Negacao(exp) => {
                let resultado_exp = exp.avaliar()?;

                return Some(resultado_exp * -1)
           }
           Numero(num) => {
                return Some(*num);
           }
        }
    }

    // Todo: refazer método de imprimir para não imprimir tanto parêntese (levar em consideração ordem de operações usando profundidade?)
    fn imprimir(&self) {
        match self {
            Operacao(operador, exp1, exp2) => {
                let precedencia = get_precedencia(operador);
                

            }
            Negacao(exp) => {
                print!("-");
                if exp.eh_numero() {
                    exp.imprimir();
                } else {
                    print!("(");
                    exp.imprimir();
                    print!(")")
                }
            }
            Numero(num) => {
                print!("{num}")
            }
        }
    }
    
    // Todo: ajeitar impressões de nós após o útlimo ("└")
    fn imprimir_arvore(&self) {
        match self {
            Operacao(operador, exp1, exp2) => {
                println!(".");
            }
            Negacao(exp) => {
                println!(".");
            }
            Numero(num) => {
                println!("{num}")
            }
        }
    }
    
    fn eh_numero(&self) -> bool {
        matches!(self, Numero(_))
    }
}

fn get_precedencia(operador: &char) -> u8 {
    match operador {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0
    }
}

// Retorna array de expressões para testes
fn get_testes() -> [Expressao; 5] {
    let testes: [Expressao; 5] = [
        // "10 + 20"
        Expressao::Operacao(
            '+',
            Box::new(Expressao::Numero(10)),
            Box::new(Expressao::Numero(20)),
        ),
        
        // "10 / 0"
        Expressao::Operacao(
            '/',
            Box::new(Expressao::Numero(10)),
            Box::new(Expressao::Numero(0)),
        ),
        
        // "(10 + 20) * 30"
        Expressao::Operacao(
            '*',
            Box::new(Expressao::Operacao(
                '+',
                Box::new(Expressao::Numero(10)),
                Box::new(Expressao::Numero(20)),
            )),
            Box::new(Expressao::Numero(30)),
        ),
        
        // "10 + 20 * 30"
        Expressao::Operacao(
            '+',
            Box::new(Expressao::Numero(10)),
            Box::new(Expressao::Operacao(
                '*',
                Box::new(Expressao::Numero(20)),
                Box::new(Expressao::Numero(30)),
            )),
        ),
        
        // "(-(10 + 20) + 30 + 40 + (50 + 60)) * -5"
        Expressao::Operacao(
            '*',
            Box::new(Expressao::Operacao(
                '+',
                Box::new(Expressao::Operacao(
                    '+',
                    Box::new(Expressao::Operacao(
                        '+',
                        Box::new(Expressao::Negacao(
                            Box::new(Expressao::Operacao(
                                '+',
                                Box::new(Expressao::Numero(10)),
                                Box::new(Expressao::Numero(20)),
                            )),
                        )),
                        Box::new(Expressao::Numero(30)),
                    )),
                    Box::new(Expressao::Numero(40)),
                )),
                Box::new(Expressao::Operacao(
                    '+',
                    Box::new(Expressao::Numero(50)),
                    Box::new(Expressao::Numero(60)),
                )),
            )),
            Box::new(Expressao::Negacao(Box::new(Expressao::Numero(5)))),
        )
    ];

    return testes;
}