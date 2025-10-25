use Expressao::*;

fn main() {
    let testes = get_testes();

    for teste in testes {
        teste.imprimir();
        print!("\n");
        teste.imprimir_arvore();
        println!("\nResultado: {:?}", teste.avaliar());
        print!("\n");
    }

}

enum Expressao {
    Operacao(char, Box<Expressao>, Box<Expressao>),
    Negacao(char, Box<Expressao>),
    Numero(i64)
}

impl Expressao {
    fn eh_numero(&self) -> bool {
        matches!(self, Numero(_))
    }

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
           Negacao(_, exp) => {
                let resultado_exp = exp.avaliar()?;

                return Some(resultado_exp * -1)
           }
           Numero(num) => {
                return Some(*num);
           }
        }
    }

    // Todo: refazer método de imprimir para não imprimir tanto parêntese (levar em consideração ordem de operações)
    fn imprimir(&self) {
        match self {
            Operacao(operador, exp1, exp2) => {
                if exp1.eh_numero() {
                    exp1.imprimir();
                } else {
                    print!("(");
                    exp1.imprimir();
                    print!(")")
                }
                print!(" {operador} ");
                if exp2.eh_numero() {
                    exp2.imprimir();
                } else {
                    print!("(");
                    exp2.imprimir();
                    print!(")")
                }
            }
            Negacao(negacao, exp) => {
                print!("{negacao}");
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
    
    // Todo: implementar isso
    fn imprimir_arvore(&self) {
        print!("Arvore aqui...")
    }
}

// Retorna array de expressões para testes
fn get_testes() -> [Expressao; 5] {
    let testes: [Expressao; 5] = [
        // "10 + 20"
        Operacao(
            '+',
            Box::new(Numero(10)),
            Box::new(Numero(20))
        ),
        
        // "10 / 0"
        Operacao(
            '/',
            Box::new(Numero(10)),
            Box::new(Numero(0))
        ),
        
        // "(10 + 20) * 30"
        Operacao(
            '*',
            Box::new(Operacao(
                '+',
                Box::new(Numero(10)),
                Box::new(Numero(20))
            )),
            Box::new(Numero(30))
        ),
        
        // "10 + 20 * 30"
        Operacao(
            '+',
            Box::new(Numero(10)),
            Box::new(Operacao(
                '*',
                Box::new(Numero(20)),
                Box::new(Numero(30))
            ))
        ),
        
        // "(-(10 + 20) + 30 + 40 + (50 + 60)) * -5"
        Operacao(
            '*',
            Box::new(Operacao(
                '+',
                Box::new(Operacao(
                    '+',
                    Box::new(Operacao(
                        '+',
                        Box::new(Negacao(
                            '-',
                            Box::new(Operacao(
                                '+',
                                Box::new(Numero(10)),
                                Box::new(Numero(20))
                            ))
                        )),
                        Box::new(Numero(30))
                    )),
                    Box::new(Numero(40))
                )),
                Box::new(Operacao(
                    '+',
                    Box::new(Numero(50)),
                    Box::new(Numero(60))
                ))
            )),
            Box::new(Negacao('-', Box::new(Numero(5))))
        )
    ];

    return testes;
}