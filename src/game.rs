pub fn rotate_board_game(game: &mut[[i32; 4]; 4]) {
    let n = 4;
    for i in 0..(4/2) {
        for j in i..n-i-1 {
            let aux = game[i][j];
            game[i][j] = game[j][n-i-1];
            game[j][n-i-1] = game[n-i-1][n-j-1];
            game[n-i-1][n-j-1] = game[n-j-1][i];
            game[n-j-1][i] = aux;
        }
    }
}

pub fn is_winner_game(game: &[[i32;4]; 4]) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if game[x][y] == 2048 {
                return true;
            }
        }
    }

    return false;
}

fn print_number_with_pad(number: i32, last: bool) {
    if(number < 10) {
        print!("   {}", number);
    } else if(number < 100) {
        print!("  {}", number);
    } else if(number < 1000) {
        print!(" {}", number);
    } else {
        print!("{}", number);
    }

    if !last {
        print!(" | ");
    }
}

pub fn print_board_game(game: &mut[[i32; 4]; 4]) {
    for y in 0..4 {
        for x in 0..4 {
            print_number_with_pad(game[x][y], x == 3);
        }

        if y != 4 {
            println!("");
            println!("-------------------------");
        }
    }
}
