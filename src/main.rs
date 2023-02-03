type Board = [[char; 3]; 3];

#[derive(Clone)]
struct Game {
    board: Board,
}

impl Game {
    fn available_moves(&self) -> Vec<[usize;2]>  {
        let mut moves: Vec<[usize; 2]> = vec![];
        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y] == '_' {
                    moves.push([x, y]);
                }
            }
        }
        return moves
    }
}

fn main() {
    const PLAYER_1:char = 'X';
    const PLAYER_2:char = 'O';
    const BOT:char = 'X';

    let board: Board = [['_','_','_'],['_','_','_'],['_','_','_']];
    let game = Game{board};

    play_game(game, PLAYER_1, PLAYER_2, BOT);
}

fn available_moves(board: &Board) -> Vec<[usize;2]>  {
    let mut moves: Vec<[usize; 2]> = vec![];
    for x in 0..3 {
        for y in 0..3 {
            if board[x][y] == '_' {
                moves.push([x, y]);
            }
        }
    }
    return moves
}
fn get_utility_map(game: &Game, player: char, next_player: char) -> Vec<([usize; 2], i32)> {
    let mut utility_map: Vec<([usize; 2], i32)> = vec![];
    let available_moves = game.available_moves();
    for i in 0..available_moves.len() {
        let next_move = available_moves[i];
        let utility = get_utility(&game, player, next_player, next_move);
        utility_map.push((next_move ,utility));
    }

    return utility_map 
}

fn get_utility(game: &Game, player: char, next_player: char, next_move: [usize; 2]) -> i32 {
    let [x, y] = next_move;
    let mut next_board = game.board.clone();
    next_board = play(next_board, player, x, y).unwrap();
    let next_game = Game{board: next_board};
    let available_moves = available_moves(&next_game.board);
    if available_moves.len() == 0 {
        let winner = check_win(&game.board);
        let utility: i32;
        match winner {
            'X' => utility = 1,
            'O' => utility = -1,
            _ => utility = 0,
        }
        return utility;
    }
    let utilities: Vec<([usize; 2], i32)> = get_utility_map(&next_game, next_player, player);

    let max;
    if player == 'X' {
        max = utilities.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    } else {
        max = utilities.iter().max_by(|a, b| b.1.cmp(&a.1)).unwrap();
    }


    return max.1;
}

fn play_game(mut game: Game, player: char, next_player: char, bot: char) {
    let mut utility_map = get_utility_map(&game, player, next_player);
    let winner = check_win(&game.board);
    let full = check_full(&game.board);
    if full {
        println!("Board is full");
        log_board(&game.board);
        return
    }
    if winner == '_' && !full {
        if player == 'X' {
            utility_map.sort_by(|a, b| b.1.cmp(&a.1));
            let bot_move: [usize; 2];
            bot_move = utility_map[0].0;
            let [x, y] = bot_move;
            game.board = play(game.board, player, x, y).unwrap();
            return play_round(game, next_player, player, bot);
        } else {
            return play_round(game, player, next_player, bot);
        }
    }
    println!("The winner is {}", winner);
    log_board(&game.board);
    return
}

fn play_round(mut game: Game, player: char, next_player: char, bot: char) {
    log_board(&game.board);
    println!("Player {} turn", player); 
    let [x, y] = read_input();
    let result = play(game.board, player, x, y);
    match result {
        Ok(v) => game.board = v,
        Err(_e) => return play_game(game, player, next_player, bot),
    } 
    let winner = check_win(&game.board);
    let full = check_full(&game.board);
    if winner == '_' && !full {
        play_game(game, next_player, player, bot); 
        return
    }
    if full {
        println!("Board is full");
        log_board(&game.board);
        return
    }
    println!("The winner is {}", winner);
    log_board(&game.board);
    return
}


fn log_board(board: &Board) {
    for i in 0..3 {
        let row = format!("{}: {}|{}|{}",i , board[i][0], board[i][1], board[i][2]);
        println!("{}", row)
    }
    println!("   0 1 2");
}

fn read_input() -> [usize; 2] {
    let mut input = String::new();
    let choice = std::io::stdin().read_line(&mut input).unwrap();
    let x: u8;
    let y: u8;
    if choice == 3 {
        y = input.as_bytes()[0] - '0' as u8;
        x = input.as_bytes()[1] - '0' as u8;
    } else if choice == 4 {
        y = input.as_bytes()[0] - '0' as u8;
        x = input.as_bytes()[2] - '0' as u8;
    } else {
        println!("Unrecognised input, please enter you chouce in the form of 'xy', 'x,y' or 'x y'");
        return read_input()
    }
    [x.into(), y.into()]
}

fn play(mut board: Board, player: char, x:usize, y:usize) -> Result<Board, &'static str> {
    if x > 2 || y > 2 {
        println!("Out of bounds, indexes start at 0");
        return Err("Out of bounds");
    }
    if board[x][y] == '_' {
        board[x][y] = player;
        Ok(board)
    } else {
        println!("move already taken, please select another spot");
        return Err("Move already taken");
    }
    
}

fn check_win(board: &Board) -> char {
    // Check rows
    for i in 0..3 {
        // println!("{}, {} , {}",board[i][0] == board[i][1], board[i][1] == board[i][2], board[i][0] != '_');
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != '_' {
            return board[i][0];
        }
    }

    // Check columns
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != '_' {
            return board[0][i];
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != '_' {
        return board[0][0];
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != '_' {
        return board[0][2];
    }

    return '_';
}

fn check_full(board: &Board) -> bool {
    for i in 0..3{
        for j in 0..3 {
            if board[i][j] == '_' {
                return false
            }
        }
    }
    return true
}
