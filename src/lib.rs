mod game;

#[test]
fn slide_array() {
    let mut game: [i32; 4] = [2,2,2,2];
    game::slide_array(&mut game);

    assert!(game[0] == 4);
}

#[test]
fn replace_random_zero_in_2_or_4_in_game() {
    let mut game: [[i32; 4]; 4] = [
        [2,2,2,2],
        [2,2,2,2],
        [2,2,2,2],
        [0,2,2,2]
    ];

    game::add_number(&mut game);
    assert!(
        ((game[3][0] == 2) || (game[3][0] == 4))
    );
}

#[test]
fn test_if_loose_the_game() {
    let game: [[i32; 4]; 4] =
    [
        [2,2,2,2],
        [2,2,2,2],
        [2,2,2,2],
        [2,2,2,2]
    ];

    assert_eq!(game::is_looser_game(&game), true);
}

#[test]
fn test_if_not_loose_the_game() {
    let game: [[i32; 4]; 4] =
    [
        [2,2,2,2],
        [2,2,2,2],
        [2,2,2,2],
        [2,2,2,2]
    ];

    assert_eq!(game::is_looser_game(&game), true);
}

#[test]
fn test_if_not_win_the_game() {
    let game: [[i32; 4]; 4] =
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ];

    assert_eq!(game::is_winner_game(&game), false);
}


#[test]
fn test_if_win_the_game() {
    let game: [[i32; 4]; 4] =
    [
        [0,2048,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ];

    assert!(game::is_winner_game(&game));
}

#[test]
fn test_rotate_board() {

    /*
      You must rotate the array in your mind. The first item is column and the second is the line.
    In draw board this game is like this:

       1 |    2 |    3 |    4
       5 |    6 |    7 |    8
       9 |   10 |   11 |   12
      13 |   14 |   15 |   16

    */
    let mut game: [[i32; 4]; 4] =
    [
        [1,5,9,13],
        [2,6,10,14],
        [3,7,11,15],
        [4,8,12,16]
    ];

    /*
      Response (in draw board):

      13 |    9 |    5 |    1
      14 |   10 |    6 |    2
      15 |   11 |    7 |    3
      16 |   12 |    8 |    4

      Array:

        [13,14,15,16],
        [9,10,11,12],
        [5,6,7,8],
        [1,2,8,4]      

     */
    game::rotate_board_game(&mut game);
    assert_eq!(game[0][0], 13);
    assert_eq!(game[0][1], 14);
    assert_eq!(game[0][2], 15);
    assert_eq!(game[0][3], 16);

    assert_eq!(game[1][0], 9);
    assert_eq!(game[1][1], 10);
    assert_eq!(game[1][2], 11);
    assert_eq!(game[1][3], 12);

    assert_eq!(game[2][0], 5);
    assert_eq!(game[2][1], 6);
    assert_eq!(game[2][2], 7);
    assert_eq!(game[2][3], 8);

    assert_eq!(game[3][0], 1);
    assert_eq!(game[3][1], 2);
    assert_eq!(game[3][2], 3);
    assert_eq!(game[3][3], 4);
}

