use std::{fmt, cmp::PartialEq};
use rand::{ thread_rng, Rng };
use std::rc::Rc;

pub enum Piece{
    User,
    Npc,
    Clear,
}

impl Piece{
    fn as_str(&self) -> &'static str {
        return match *self{
                Piece::User => "User",   
                Piece::Npc => "Npc",    
                Piece::Clear => "Clear",    
        } 
    }
}

impl Eq for Piece{}
impl PartialEq for Piece{
    fn eq(&self, other: &Piece) -> bool{
        return self == other;
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
impl Copy for Piece{}
impl Clone for Piece{
    fn clone(&self) -> Piece {
        return *self;
    }
}

struct PlaySelector{
    pub piece: Piece,
    pub position: usize, 
    //prefer u8 here but working around small constraint.
    //This echoes elsewhere though. Careful not to repeat
    //in the future.
    //iter preferable. Slice indexing reqs usize as arg.
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
               (1, 2, 3),
               (4, 5, 6),
               (7, 8, 9),
               (1, 4, 7),
               (2, 5, 8),
               (3, 6, 9),
               (1, 5, 9),
               (3, 5, 7),
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
        /*
        if new_play.piece == self.positions[new_play.position] {
            return Err("Illegal play: space occupied.");
        }*/
        
        let mut positions_for_new: [Piece; 9] = [Piece::Clear; 9]; 
        
        for i in 0..9 {
            positions_for_new[i] = self.positions[i];      
        }
        positions_for_new[new_play.position] = new_play.piece;
         
        return Ok(
                TableState{
                    positions: positions_for_new,
                    player: Piece::User,
                }
            );
    }

    fn is_win(&self, win_options: &WinOptions) -> bool{
        for win_option in win_options.options(){
            if self.positions[win_option.0] != Piece::Clear
                && self.positions[win_option.0]
                == self.positions[win_option.1]
                && self.positions[win_option.1]
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
pub struct GameMaster{
    win_options: WinOptions,
    game_history: Vec::<TableState>,
     
}

impl GameMaster{
    pub fn new() -> GameMaster {
        let win_options = WinOptions::new();
        let mut game_history = Vec::<TableState>::new();
        game_history.push(TableState::new());
        
        return GameMaster{
            win_options,
            game_history, 
        }
    }

    pub fn add_move(&mut self, piece: Piece, position: usize){
        let new_play = PlaySelector::new(piece, position);
        self.game_history.push(
            self.game_history.last().unwrap().duplicate_with_new(new_play)
                .unwrap());
    }

    pub fn npc_random_move(&mut self){
        let mut  open_positions = Vec::<Rc::<Piece>>::new(); 
        for piece in *self.game_history.last().unwrap().positions(){
            if piece == Piece::Clear{
                open_positions.push(Rc::new(piece));
            }
        } 
        let mut rng = thread_rng(); 
        let selected_pos: usize = rng.gen_range(0..(open_positions.len()));
        self.add_move(Piece::Npc, selected_pos); 
    }
    
    pub fn backtrack(&mut self, jumps: u8) -> Result<(), &'static str>{
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

    pub fn check_win(&self) -> bool {
        return self.game_history.last().unwrap().is_win(&self.win_options);
    }
   
    pub fn print_table(&self, reversed_history_index: isize) -> Result<(), &'static str>{
        let history_index = (self.game_history.len() as isize - reversed_history_index) - 1;
        if history_index < 0 {
            return Err("Request beyond acceptable range.");
        }
        let reference_table = &self.game_history[history_index as usize];
        let t_positions = reference_table.positions();
        let l = String::from("_"); //Should find a way around this.
        println!("Player: {}", reference_table.player.as_str()); 
        
        println!("\t {} | {} | {} ", t_positions[0], t_positions[1], t_positions[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[3], t_positions[4], t_positions[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[6], t_positions[7], t_positions[8]);
        return Ok(());
    }
}
