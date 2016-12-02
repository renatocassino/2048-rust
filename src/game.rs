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

fn print_line(line: &mut[i32; 4]) {
    println!("  {} | {} | {} | {}", line[0], line[1], line[2], line[3]);
}

fn print_separator() {
    println!("-------------------");
}

pub fn print_board_game(game: &mut[[i32; 4]; 4]) {
    for n in 0..4 {
        print_line(&mut game[n]);
        if n != 4 {
            print_separator();
        }
    }
}
