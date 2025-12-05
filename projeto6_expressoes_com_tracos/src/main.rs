use Expressao::*;

fn main() {
    let testes = get_testes();
    for teste in testes {
        teste.emitir();
        println!("Resultado: {:?}", teste.avaliar());
        println!("-------------------");
    }
}

enum Expressao {
    Binaria {
        esquerda: Box<Expressao>,
        operador: String,
        direita: Box<Expressao>,
    },
    Unaria (Box<Expressao>),
    Numero(i64),
}

trait ExpressaoTrait {
    fn avaliar(&self) -> Option<i64>;
    fn emitir(&self);
}

impl ExpressaoTrait for Expressao {
    fn avaliar(&self) -> Option<i64> {
        match self {
            Numero(valor) => Some(*valor),
            Unaria(expr) => {
                let result_expr = expr.avaliar()?;
                return Some(result_expr * -1)
            },
            Binaria {esquerda, operador, direita} => {
                let result_e = esquerda.avaliar()?;
                let result_d = direita.avaliar()?;

                match operador.as_str() {
                    "+" => Some(result_e + result_d),
                    "-" => Some(result_e - result_d),
                    "*" => Some(result_e * result_d),
                    "/" => {
                        if result_d == 0 {
                            None
                        } else {
                            Some(result_e / result_d)
                        }
                    },
                    "%" => {
                        if result_d == 0 {
                            None
                        } else {
                            Some(result_e / result_d)
                        }
                    },
                    _ => None,
                }
            }
        }
    }

    fn emitir(&self) {
        match self {
            Expressao::Numero(valor) => println!("Empilhar {}", valor),
            Expressao::Unaria(expr) => {
                expr.emitir();
                println!("negar");
            }
            Expressao::Binaria { esquerda, operador, direita } => {
                esquerda.emitir();
                direita.emitir();
                match operador.as_str() {
                    "+" => println!("adicionar"),
                    "-" => println!("subtrair"),
                    "*" => println!("multiplicar"),
                    "/" => {
                        println!("dividir")
                    },
                    "%" => {
                        println!("obter resto")
                    },
                    _ => println!("Operador desconhecido"),
                }
            }
        }
    }
}

fn get_testes() -> [Expressao; 5] {
    [
        // 10 + 20
        Binaria {
            esquerda: Box::new(Numero(10)),
            operador: "+".to_string(),
            direita: Box::new(Numero(20)),
        },
        // 10 / 0
        Binaria {
            esquerda: Box::new(Numero(10)),
            operador: "/".to_string(),
            direita: Box::new(Numero(0)),
        },
        // (10 + 20) * 30
        Binaria {
            esquerda: Box::new(Binaria {
                esquerda: Box::new(Numero(10)),
                operador: "+".to_string(),
                direita: Box::new(Numero(20)),
            }),
            operador: "*".to_string(),
            direita: Box::new(Numero(30)),
        },
        // 10 + 20 * 30
        Binaria {
            esquerda: Box::new(Numero(10)),
            operador: "+".to_string(),
            direita: Box::new(Binaria {
                esquerda: Box::new(Numero(20)),
                operador: "*".to_string(),
                direita: Box::new(Numero(30)),
            }),
        },
        // (-(10 + 20) + 30 + 40 + (50 + 60)) * -5
        Binaria {
            esquerda: Box::new(Binaria {
                esquerda: Box::new(Binaria {
                    esquerda: Box::new(Binaria {
                        esquerda: Box::new(Unaria(Box::new(Binaria {
                            esquerda: Box::new(Numero(10)),
                            operador: "+".to_string(),
                            direita: Box::new(Numero(20)),
                        }))),
                        operador: "+".to_string(),
                        direita: Box::new(Numero(30)),
                    }),
                    operador: "+".to_string(),
                    direita: Box::new(Numero(40)),
                }),
                operador: "+".to_string(),
                direita: Box::new(Binaria {
                    esquerda: Box::new(Numero(50)),
                    operador: "+".to_string(),
                    direita: Box::new(Numero(60)),
                }),
            }),
            operador: "*".to_string(),
            direita: Box::new(Unaria(Box::new(Numero(5)))),
        },
    ]
}