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

// fn get_profundidade(exp_pai: &Expressao, exp_filho:Box<Expressao>, profundidade: u8) -> Option<u8> {
//     match exp_pai {
//         Operacao(_, exp1, exp2) => {
//             if *exp1 == exp_filho || *exp2 == exp_filho {
//                 return Some(profundidade);
//             }
//             let 
//             get_profundidade(exp1, exp_filho, profundidade + 1)
//         }
//         Negacao(_, exp) => {
//             if *exp == exp_filho {
//                 return Some(profundidade)
//             }
//             get_profundidade(exp, exp_filho, profundidade + 1)
//         }
//         Numero(_) => {
//             None
//         }
//     }
// }

//#[derive(PartialEq)]
enum Expressao {
    Operacao(char, Box<Expressao>, Box<Expressao>, u8), // char: operador, u8: profundidade 
    Negacao(char, Box<Expressao>, u8), // char: negacao (-), u8: profundidade
    Numero(i64, u8) // i64: numero, u8: profundidade
}

impl Expressao {
    fn eh_numero(&self) -> bool {
        matches!(self, Numero(_, _))
    }

    fn avaliar(&self) -> Option<i64> {
        match self {
            Operacao(operador, exp1, exp2, _) => {
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
           Negacao(_, exp, _) => {
                let resultado_exp = exp.avaliar()?;

                return Some(resultado_exp * -1)
           }
           Numero(num, _) => {
                return Some(*num);
           }
        }
    }

    // Todo: refazer método de imprimir para não imprimir tanto parêntese (levar em consideração ordem de operações usando profundidade?)
    fn imprimir(&self) {
        match self {
            Operacao(operador, exp1, exp2, _) => {
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
            Negacao(negacao, exp, _) => {
                print!("{negacao}");
                if exp.eh_numero() {
                    exp.imprimir();
                } else {
                    print!("(");
                    exp.imprimir();
                    print!(")")
                }
            }
            Numero(num, _) => {
                print!("{num}")
            }
        }
    }
    
    // Todo: ajeitar impressões de nós após o útlimo ("└")
    fn imprimir_arvore(&self) {
        match self {
            Operacao(operador, exp1, exp2, profundidade) => {
                println!("{operador}");
                for _i in 0..*profundidade {
                    print!("| ");
                }
                print!("├ ");
                exp1.imprimir_arvore();
                for _i in 0..*profundidade {
                    print!("| ");
                }
                print!("└ ");
                exp2.imprimir_arvore();
            }
            Negacao(negacao, exp, profundidade) => {
                println!("{negacao}");
                for _i in 0..*profundidade {
                    print!("| ");
                }
                print!("└ ");
                exp.imprimir_arvore();
            }
            Numero(num, _profundidade) => {
                println!("{num}")
            }
        }
    }
}

// Retorna array de expressões para testes
fn get_testes() -> [Expressao; 5] {
    let testes: [Expressao; 5] = [
        // "10 + 20"
        Expressao::Operacao(
            '+',
            Box::new(Expressao::Numero(10, 1)),
            Box::new(Expressao::Numero(20, 1)),
            0
        ),
        
        // "10 / 0"
        Expressao::Operacao(
            '/',
            Box::new(Expressao::Numero(10, 1)),
            Box::new(Expressao::Numero(0, 1)),
            0
        ),
        
        // "(10 + 20) * 30"
        Expressao::Operacao(
            '*',
            Box::new(Expressao::Operacao(
                '+',
                Box::new(Expressao::Numero(10, 2)),
                Box::new(Expressao::Numero(20, 2)),
                1
            )),
            Box::new(Expressao::Numero(30, 1)),
            0
        ),
        
        // "10 + 20 * 30"
        Expressao::Operacao(
            '+',
            Box::new(Expressao::Numero(10, 1)),
            Box::new(Expressao::Operacao(
                '*',
                Box::new(Expressao::Numero(20, 2)),
                Box::new(Expressao::Numero(30, 2)),
                1
            )),
            0
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
                            '-',
                            Box::new(Expressao::Operacao(
                                '+',
                                Box::new(Expressao::Numero(10, 6)),
                                Box::new(Expressao::Numero(20, 6)),
                                5
                            )),
                            4
                        )),
                        Box::new(Expressao::Numero(30, 4)),
                        3
                    )),
                    Box::new(Expressao::Numero(40, 3)),
                    2
                )),
                Box::new(Expressao::Operacao(
                    '+',
                    Box::new(Expressao::Numero(50, 3)),
                    Box::new(Expressao::Numero(60, 3)),
                    2
                )),
                1
            )),
            Box::new(Expressao::Negacao('-', Box::new(Expressao::Numero(5, 2)), 1)),
            0
        )
    ];

    return testes;
}