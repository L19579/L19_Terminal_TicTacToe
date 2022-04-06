use std::collections::HashMap;
use l19_terminal_tictactoe as l19;

fn main() {
    //TODO:
    //[x] Specify input bindings
    //[-] Assing player sign with random selector.
    //[ ] Gameplay loop:
    //  [ ] Check for quit prompt
    //  [ ] Check for backtrack request
    //  [ ] Print table  
    //  [ ] Take player input, print updated table
    //  [ ] Submit AI input, print updated RustcDecodable  
    //  [ ] Check for Win:
    //      [ ] If true announce winner and exit loop.
    //[ ] Announce game exit.
    

    let input_bindings = HashMap::from([
            ("a1", "1"),
            ("a2", "2"),
            ("a3", "3"),
            ("b1", "4"),
            ("b2", "5"),
            ("b3", "6"),
            ("c1", "7"),
            ("c2", "8"),
            ("c3", "9"),
    ]); 
    
    println!("Launching L19_Terminal_TicTacToe");
    println!("Write \"quit\" at any time to exit");

    let user_input = String::new();
    let mut gm = l19::GameMaster::new();
    for i in 0..9{
        gm.npc_random_move();
        gm.print_table(0).unwrap();
    
    }

   // loop{
        
   // }
    

}
