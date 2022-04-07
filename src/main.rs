use rand::{ Rng, thread_rng };
use std::collections::HashMap;
use l19_terminal_tictactoe as l19;

fn main() {
    //TODO:
    //[x] Specify input bindings
    //[x] Assign starting player with random selector.
    //[x] Gameplay loop:
    //  [x] Check for quit prompt, and execute
    //  [-] Check for backtrack request
    //      >> Dropped, logic is getting bloated.
    //  [x] Print table  
    //  [x] Take player input, print updated table
    //  [x] Submit AI input, print updated table.  
    //  [x] Check for Win:
    //      [x] If true announce winner and exit loop.
    //[x] Announce game exit.
    let mut final_statement = String::from("Tie match! Thanks for playing."); 
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
    println!("\nHere are you input bindings: ");
    let mut gm = l19::GameMaster::new(&input_bindings);
    println!("\n\n");
    gm.show_labeled_table();
    let mut current_player: l19::Piece = 
        l19::Piece::select_with_int(m_rng.gen_range(0..2));
        // Explain key bindings here.
    println!("{} has first move!", current_player.as_str()); 
    current_player = current_player.opposite();
    'top_loop: for _ in 0..9 {
        for attempt in 1..6 {
            match gm.next_mover_w_prompt(&current_player){
                Ok(()) => break,
                Err(e) => {
                    eprintln!("{}", e);
                    if e.trim() == "quit" {
                        final_statement = String::from("Stopping the game.");
                        break 'top_loop;
                    }
                    if attempt == 5{
                        eprintln!("{}", e);
                        panic!();
                    }
                    println!("Input binding must be exact, and move legal.");
                    println!
                        ("Let's give that another try (attempt {}/5): ", attempt);
                    continue;
                },
            }
        }
        current_player = current_player.opposite();
        
        let (is_win, who) = gm.check_win();
        if is_win {
            final_statement = String::from(format!("{} Wins! Thanks for playing.", who.as_str()));
            break;
        }
    }
    println!("\n{}", final_statement);
}
