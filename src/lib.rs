use rand::{ Rng, thread_rng };
use std::{
    io::{Write, stdin,},
    cmp::PartialEq,
    collections::HashMap,
    time::Duration,
    thread, //heavy
    fmt, 
};

pub enum Piece{
    User,
    Npc,
    Clear,
}

impl Piece{
    pub fn as_str(&self) -> &'static str {
        return match *self{
                Piece::User => "User",   
                Piece::Npc => "Npc",    
                Piece::Clear => "Clear",    
        } 
    }

    pub fn opposite(&self) -> Piece {
        match *self {
            Piece::User => Piece::Npc,
            Piece::Npc => Piece::User,
            _ => panic!("Invalid input."),
        } 
    }
    
    pub fn select_with_int(choice: u8) -> Piece {
        match choice {
            0 => Piece::User,
            1 => Piece::Npc,
            _ => Piece::Clear,
        }
    }
}

impl Eq for Piece{}
impl PartialEq for Piece{
    fn eq(&self, other: &Piece) -> bool{
        //Easy to make the obv error here. Tracking runtime
        //bugs on a large codebase, not so much.
        return self.as_str() == other.as_str();
    }
}

impl Copy for Piece{}
impl Clone for Piece{
    fn clone(&self) -> Piece {
        return *self;
    }
}


impl fmt::Display for Piece{
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> fmt::Result{
            return match *self{
                Piece::User => write!(formatter, "X"),    
                Piece::Npc => write!(formatter, "O"),    
                Piece::Clear => write!(formatter, " "),    
            };
        }
    
}

struct PlaySelector{
    pub piece: Piece,
    pub position: usize, 
    //usize to avoid numerous cast calls for indexing.
    //u8 preferable.
}

impl PlaySelector{
    fn new (piece: Piece, position: usize) -> PlaySelector {
        return PlaySelector {
            piece,
            position,
        };
    }
}

struct WinOptions{
    options: [(usize, usize, usize); 8], 
}

impl WinOptions{
    fn new() -> WinOptions {
       return WinOptions{
           options: [
               (0, 1, 2),
               (3, 4, 5),
               (6, 7, 8),
               (0, 3, 6),
               (1, 4, 7),
               (2, 5, 8),
               (0, 4, 8),
               (2, 4, 6),
           ],
       };   
    }

    fn options(&self) -> [(usize, usize, usize); 8] {
        return self.options;
    }
}

struct TableState{
    positions: [Piece; 9],
    player: Piece,
}

impl TableState{
    fn new() -> TableState {
        return TableState{
            positions: [Piece::Clear; 9],
            player: Piece::Clear,
        };
    }

    fn positions(&self) -> &[Piece; 9]{
        return &self.positions;
    }
    
    fn player(&self) -> &Piece{
        return &self.player
    }
    
    fn duplicate_with_new(&self, new_play: PlaySelector) 
        -> Result<TableState, &'static str >{
        
        if self.positions[new_play.position] != Piece::Clear{
            return Err("Illegal play: space occupied.");
        }
        let mut positions_for_new: [Piece; 9] = [Piece::Clear; 9]; 
        
        for i in 0..9 {
            positions_for_new[i] = self.positions[i];      
        }
        positions_for_new[new_play.position] = new_play.piece;
         
        return Ok(
                TableState{
                    positions: positions_for_new,
                    player: new_play.piece,
                }
            );
    }

    fn is_win(&self, win_options: &WinOptions) -> bool{
        for win_option in win_options.options(){
            
            if self.positions[win_option.0] != Piece::Clear
                && self.positions[win_option.0]
                == self.positions[win_option.1]
                && self.positions[win_option.0]
                == self.positions[win_option.2]{
                return true;
            }
        }
        return false;
    }
}

impl Copy for TableState{}
impl Clone for TableState{
    fn clone(&self) -> TableState{
        return *self;
    }
}

//interface to main.rs
pub struct GameMaster <'a>{
    win_options: WinOptions,
    game_history: Vec::<TableState>,
    key_bindings: &'a HashMap<&'a str, usize>
    //key_bindings are now a req.  
}

impl<'a> GameMaster<'a>{
    pub fn new(key_bindings_c: &'a HashMap<&'a str, usize>) -> GameMaster<'a> {
        let win_options = WinOptions::new();
        let mut game_history = Vec::<TableState>::new();
        game_history.push(TableState::new());
        
        return GameMaster{
            win_options,
            game_history,
            key_bindings: key_bindings_c,
        }
    }

    pub fn next_mover_w_prompt
        (&mut self, last_piece: &Piece) 
            -> Result<(), &'static str> { 
        match *last_piece {
            Piece::User => {
                print!("Npc is thinking");
                for _ in 0..4 {
                    std::io::stdout().flush().unwrap();
                    print!(".");
                    thread::sleep(Duration::from_millis(500));
                }
                println!();
                self.npc_random_move();
            }, 
            Piece::Npc => {
                let mut user_input = String::new();
                print!("User's turn: ");
                match stdin().read_line(&mut user_input){
                    Ok(_) => (),
                    Err(_) => {
                        return Err("Error reading line.)");
                    },
                };
                let user_input: &str = user_input.trim(); 
                if user_input == "quit"{ 
                    return Err("quit");
                };
                
                let converted_input : usize = 
                    match self.key_bindings.get(user_input){
                        Some(input) => {
                            *input
                        },
                        None => {
                            return Err("Invalid input: binding error.");
                        },
                };
                
                match self.add_move(Piece::User, converted_input) {
                    Ok(()) => (),
                    Err(e) => {
                        return Err(e);
                    }
                };
            },
            _ => {
                return Err("Invalid input");
            },
        };
            self.print_table(0).unwrap();
            return Ok(());
        } 

    pub fn add_move(&mut self, piece: Piece, position: usize) -> Result <(), &'static str>{
        let new_play = PlaySelector::new(piece, position);
        self.game_history.push(
            match self.game_history.last().unwrap().duplicate_with_new(new_play){
                Ok(new_table) => new_table,
                Err (e) => {
                    return Err(e);
                },
            });
        
        return Ok(());
    }

    pub fn npc_random_move(&mut self){
        let mut open_positions = Vec::<usize>::new();
        let mut counter = 0;
        for piece in *self.game_history.last().unwrap().positions(){
            if piece == Piece::Clear{
                open_positions.push(counter);
            }
            counter += 1;
        } 
        let mut rng = thread_rng(); 
        let rng_pick: usize = rng.gen_range(0..(open_positions.len()));
        self.add_move(Piece::Npc, open_positions[rng_pick]).unwrap(); 
    }
    
    //This fn is not in use. Will have it running in v2. Overkill atm.
    pub fn back_track(&mut self, jumps: u8) -> Result<(), &'static str>{
    //It's on cleaning up type casts in v2.
    //TODO: Potential unhandled bugs
        if jumps < 1 as u8 ||  
            jumps*2 >= (self.game_history.len() - 2) as u8 {
            return Err("Backtrack: out of range request.");
        }
        
        self.game_history.truncate((jumps*2)as usize); 
        println!("Jumping back to play #{}", self.game_history.len());
        return Ok(());
    }

    pub fn check_win(&self) -> (bool, &Piece) {
        return (self.game_history.last().unwrap().is_win(&self.win_options),
            self.game_history.last().unwrap().player());
    }
   
    pub fn print_table(&self, reversed_history_index: isize) -> Result<(), &'static str>{
        let history_index = (self.game_history.len() as isize - reversed_history_index) - 1;
        if history_index < 0 {
            return Err("Request beyond acceptable range.");
        }
        let reference_table = &self.game_history[history_index as usize];
        let t_positions = reference_table.positions();
        let l = String::from("_"); //Should clean this up.
        println!("Player: {}", reference_table.player.as_str()); 
        
        println!("\t {} | {} | {} ", t_positions[0], t_positions[1], t_positions[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[3], t_positions[4], t_positions[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[6], t_positions[7], t_positions[8]);
        return Ok(());
    }

    pub fn show_labeled_table(&self){
        let l = "_";
        let mut key : Vec::<&str> = Vec::new(); 
        for (k, _) in self.key_bindings.iter(){
            key.push(k);
        }
        key.sort();
        //Potential bug if user's allowed unrestricted liberties with bindings.
        //This doesn't work if the UTF-8s aren't chronological.
        println!("\t {}| {}| {} ", key[0], key[1], key[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {}| {}| {} ", key[3], key[4], key[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {}| {}| {} ", key[6], key[7], key[8]);
    
    }
}
