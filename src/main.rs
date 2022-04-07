use rand::{ Rng, thread_rng };
use std::collections::HashMap;
use l19_terminal_tictactoe as l19;

fn main() {
    //TODO:
    //[x] Specify input bindings
    //[x] Assign starting player with random selector.
    //[ ] Gameplay loop:
    //  [ ] Check for quit prompt
    //  [/] Check for backtrack request
    //  [x] Print table  
    //  [x] Take player input, print updated table
    //  [x] Submit AI input, print updated table.  
    //  [x] Check for Win:
    //      [x] If true announce winner and exit loop.
    //[x] Announce game exit.
    
    let mut m_rng = thread_rng();
    let input_bindings : HashMap::<&str, usize> = HashMap::from([
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
    
    println!("Launching L19_Terminal_TicTacToe");
    println!("Write \"quit\" at any time to exit"); 

    let user_input = String::new();
    let mut gm = l19::GameMaster::new(&input_bindings);
    
    'top_loop: loop{
        let mut current_player: l19::Piece = 
            l19::Piece::select_with_int(m_rng.gen_range(0..2));
            // Explain key bindings here.
        println!("{} has first move!", current_player.as_str()); 
        current_player = current_player.opposite();
        for _ in 0..9 {
            gm.next_mover_w_prompt(&current_player).unwrap();
            current_player = current_player.opposite();
            
            let (is_win, who) = gm.check_win();
            if is_win {
                println!("\n{} Wins! Thanks for playing.", who.as_str());
                break 'top_loop
            }
        }
        println!("\nTie match! Thanks for playing.");
        break;
    }
}
