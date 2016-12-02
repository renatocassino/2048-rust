mod game;

#[test]
fn test_rotate_board() {

    let mut game: [[i32; 4]; 4] =
    [
        [1,2,3,4],
        [5,6,7,8],
        [9,10,11,12],
        [13,14,15,16]
    ];

    game::rotate_board_game(&mut game);
    assert_eq!(game[0][0], 4);
    assert_eq!(game[0][1], 8);
    assert_eq!(game[0][2], 12);
    assert_eq!(game[0][3], 16);

    assert_eq!(game[1][0], 3);
    assert_eq!(game[1][1], 7);
    assert_eq!(game[1][2], 11);
    assert_eq!(game[1][3], 15);

    assert_eq!(game[2][0], 2);
    assert_eq!(game[2][1], 6);
    assert_eq!(game[2][2], 10);
    assert_eq!(game[2][3], 14);

    assert_eq!(game[3][0], 1);
    assert_eq!(game[3][1], 5);
    assert_eq!(game[3][2], 9);
    assert_eq!(game[3][3], 13);
}

