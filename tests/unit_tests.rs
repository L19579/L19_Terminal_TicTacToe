use l19_terminal_tictactoe as l19;

//Testing custom 'input bindings'.. if you will.
#[Test]
fn check_input_bindings(){
    let input_bindins : HashMap::<&str, usize> = HashMap::from([
        ("a1", 0),
        ("a2", 1),
        ("a3", 2),
        ("b1", 3),
        ("b2", 4),
        ("b3", 5),
        ("c1", 6),
        ("c2", 7),
        ("c3", 8),
    ]);

    let gm = 19::GameMaster::new();

}

//Piece legal placement in 9/9 spaces.
#[test]
fn check_legal_placement(){
    let mut gm = l19::GameMaster::new();
    for i in 0..9{
        gm.npc_random_move();
    }
}

//Detecting all possible wins sans bot
#[test]
fn check_all_wins(){
    let win_count: u8 = 0;
    let win_possibilities: [(usize, usize, usize) ; 8]
        = [
            (0, 1, 2), 
            (3, 4, 5), 
            (6, 7, 8), 
            (0, 3, 6), 
            (1, 4, 7), 
            (2, 5, 8), 
            (0, 4, 8), 
            (2, 4, 6), 
        ];

    for (a, b, c) in win_possibilities {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::User, a); 
        gm.add_move(l19::Piece::User, b); 
        gm.add_move(l19::Piece::User, c);
        let (is_win, player) = gm.check_win();
        
        if *player != l19::Piece::User {
            panic!();
        } 
        assert_eq!(is_win, true);
    }
    
    for (a, b, c) in win_possibilities {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::Npc, a); 
        gm.add_move(l19::Piece::Npc, b); 
        gm.add_move(l19::Piece::Npc, c);
        let (is_win, player) = gm.check_win();
        
        if *player != l19::Piece::Npc {
            panic!();
        } 
        assert_eq!(is_win, true);
    }

    for (a, b, c) in win_possibilities {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::Clear, a); 
        gm.add_move(l19::Piece::Clear, b); 
        gm.add_move(l19::Piece::Clear, c);
        let (is_win, player) = gm.check_win();
        
        if *player != l19::Piece::Clear {
            panic!();
        } 
        //Piece::Clear cannot win.
        assert_eq!(is_win, false);
    }
}

//Detecting non wins. Not exhaustive, too many plays.
#[test]
fn check_some_non_win(){
    let some_non_wins: [(usize, usize, usize) ; 8]
        = [
            (0, 1, 3), 
            (0, 1, 4), 
            (0, 3, 4), 
            (0, 4, 5), 
            (1, 4, 5), 
            (1, 3, 5), 
            (2, 4, 5), 
            (2, 6, 8), 
        ];

    for (a, b, c) in some_non_wins {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::User, a);
        gm.add_move(l19::Piece::User, b);
        gm.add_move(l19::Piece::User, c);
        let (is_win, _) = gm.check_win();
        assert_eq!(is_win, false);
    }    
}
