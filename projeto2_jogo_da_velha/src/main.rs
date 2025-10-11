use std::io;

fn main() {
    let mut board: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut current_player: char = 'X';
    let mut times_played: i32 = 0;
    loop {
        show_board(&board);
        if times_played >= 9 {
            println!("Jogo finalizado, empate.");
            break
        }
        match read_input() {
            Ok(coord) => {
                let (x, y) = coord;
                if board[x as usize][y as usize] != ' ' {
                    println!("Posição já utilizada.");
                    continue
                } else {
                    times_played += 1;
                    board[x as usize][y as usize] = current_player;
                    if is_game_over(&board) {
                        show_board(&board);
                        println!("Jogo finalizado, vitória de '{current_player}'");
                        break
                    } else {
                        if current_player == 'X' {
                            current_player = 'O'
                        } else {
                            current_player = 'X'
                        }
                    }
                }
            }
            Err(msg) => {
                println!("{msg}");
                continue
            }
        }
    }
}

fn show_board(board: &[[char; 3]; 3]) {
    println!(" {} | {} | {}", board[0][0], board[0][1], board[0][2]);
    println!("-----------");
    println!(" {} | {} | {}", board[1][0], board[1][1], board[1][2]);
    println!("-----------");
     println!(" {} | {} | {}", board[2][0], board[2][1], board[2][2]);
}

fn is_game_over(board: &[[char; 3]; 3]) -> bool {
    for line in board {
        if line[0] != ' ' && line[0] == line[1] && line[1] == line[2] {
            return true;
        }
    }
    for col in 0..3 {
        if board[0][col] != ' ' && 
           board[0][col] == board[1][col] && 
           board[1][col] == board[2][col] {
            return true;
        }
    }
    if board[1][1] != ' ' {
        if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            return true;
        }
        if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            return true;
        }
    }
    
    return false
}

fn int_to_coord(index: i32) -> (i32, i32) {
    let idx = index - 1; 
    return (idx / 3, idx % 3)
}

fn read_input() -> Result<(i32, i32), String> {
    let mut input = String::new();
    println!("Próxima jogada: ");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            match input.trim().parse::<i32>() {
                Ok(num_input) => {
                    if num_input < 1 || num_input > 9 {
                        return Err(format!("Número {} inválido. Informe um número entre 1-9.", num_input));
                    }
                    let coord = int_to_coord(num_input);
                    Ok(coord)
                }
                Err(_) => {
                   Err(format!("Por favor digite um número válido."))
                }
            }
        }
        Err(e) => {
            Err(format!("Erro de I/O: {}", e))
        }
    }
}