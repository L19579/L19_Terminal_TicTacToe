use std::{fmt, cmp::PartialEq};
use rand::{ Rng, thread_rng };
use std::{
    time::Duration,
    io::stdin,
    collections::HashMap,
    thread, //heavy
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

    fn piece(&self) -> &Piece {
        return &self.piece
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
    key_bindings: Option::<HashMap<&'a str, usize>>
}

impl<'a> GameMaster<'a>{
    pub fn new() -> GameMaster<'a> {
        let win_options = WinOptions::new();
        let mut game_history = Vec::<TableState>::new();
        game_history.push(TableState::new());
        
        return GameMaster{
            win_options,
            game_history,
            key_bindings: None,
        }
    }

    pub fn set_key_bindings(&mut self, choice: HashMap<&'a str, usize>)
        -> Result<(), &'static str>{
        self.key_bindings = Some(choice); 
        return Ok(());
    }
    
    pub fn next_mover_w_prompt
        (&mut self, last_piece: &Piece, key_bindings: HashMap::<&str, &str>) 
        -> Result<(), &'static str> {
        match *last_piece {
            Piece::User => {
                print!("Bot is thinking");
                for i in 0..2 {
                    print!(".");
                    thread::sleep(Duration::from_millis(500));
                }
                println!();
                self.npc_random_move();
                self.print_table(0);
                return Ok (());
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
                
                if user_input.as_str() == "quit"{ 
                    return Err("quit");
                }
                
                let converted_input : usize = 
                    match key_bindings.get(user_input.as_str()){
                        Some(input) => {
                            *input // Error here: Getting &&Str ---------------------------
                        },
                        None => {
                            return Err("Invalid input");
                        },
                };
                
                self.add_move(Piece::User, converted_input);
                return Ok(());
            },
            _ => {
                return Err("Invalid input");
            }
            self.print_table(0);

        } 
    } 

    pub fn add_move(&mut self, piece: Piece, position: usize){
        let new_play = PlaySelector::new(piece, position);
        self.game_history.push(
            self.game_history.last().unwrap().duplicate_with_new(new_play)
                .unwrap());
    }

    pub fn npc_random_move(&mut self){
        let mut open_positions = Vec::<usize>::new();
        let mut counter = 0;
        for piece in *self.game_history.last().unwrap().positions(){
            if piece == Piece::Clear{
                open_positions.push(counter);
                println!("Space {} is clear.", counter);
            }
            counter += 1;
        } 
        let mut rng = thread_rng(); 
        let rng_pick: usize = rng.gen_range(0..(open_positions.len()));
        self.add_move(Piece::Npc, open_positions[rng_pick]); 
    }
    
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
}
