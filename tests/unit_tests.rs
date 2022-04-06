use l19_terminal_tictactoe as l19;

//Bot legal placement in 9/9 spaces.
//Expected result: ok.
#[test]
fn bot_solo(){
    let mut gm = l19::GameMaster::new();
    for i in 0..9{
        gm.npc_random_move();
        gm.print_table(0).unwrap();
    }
}

//Detecting all possible wins sans bot
//Expected result: ok.
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
        
        assert_eq!(is_win, true);
        //assert_eq!(player, String::from("User"));
    }
    
    for (a, b, c) in win_possibilities {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::Npc, a); 
        gm.add_move(l19::Piece::Npc, b); 
        gm.add_move(l19::Piece::Npc, c);
        let (is_win, player) = gm.check_win();
        if *player != l19::Piece::Npc {
            panic!("yeye");
        } 
        assert_eq!(is_win, true);
        //assert_eq!(player, String::from("Npc"));
    }


    for (a, b, c) in win_possibilities {
        let mut gm = l19::GameMaster::new();
        gm.add_move(l19::Piece::Clear, a); 
        gm.add_move(l19::Piece::Clear, b); 
        gm.add_move(l19::Piece::Clear, c);
        let (is_win, player) = gm.check_win();
        
        assert_eq!(is_win, false);
        //assert_eq!(player.as_str(), String::from("Clear"));
    }

    //assert_eq!(win_count, 8);
}
