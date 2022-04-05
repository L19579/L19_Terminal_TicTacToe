use std::{fmt, cmp::PartialEq};

pub enum Piece{
    User,
    Npc,
    Clear,
}

impl Piece{
    fn sign(&self, formatter: &mut fmt::Formatter)
        -> fmt::Result{
        return match *self{
                Piece::User => write!(formatter, "X"),    
                Piece::Npc => write!(formatter, "Y"),    
                Piece::Clear => write!(formatter, " "),    
        } 
    }
}

//This is redundant. Why can't Rust compute assert_eq! for enums
//w/o explicit implementation? --> Likely an ext of lt uncertainties.
impl PartialEq for Piece{
    fn eq(&self, other: &Piece) -> bool{
        if self == other{
            return true;  
        } else {
        return false;
        }
    }
}

impl fmt::Display for Piece{
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> fmt::Result{
            return match *self{
                Piece::User => write!(formatter, "User"),    
                Piece::Npc => write!(formatter, "Npc"),    
                Piece::Clear => write!(formatter, "Clear"),    
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
    piece: Piece,
    position: usize, 
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
//Would have been simpler to use new(args) in a TableState Vec.
//Benefits of current structure don't help readability.
//See duplicate_with_new(), change with versioning. TODO.
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
    
    fn duplicate_with_new(&self, new_play: Option<PlaySelector>) 
        -> Result<TableState, &'static str >{
        let mut positions_for_new: [Piece; 9] = [Piece::Clear; 9];
        for i in 1..10 {
            positions_for_new[i] = self.positions[i];      
        }
        //Unfinished thought here. This was meant to remove new()
        //Saved versioning. None for match should change. TODO
        match new_play {
            Some(played) => {
                if positions_for_new[played.position] != Piece::Clear
                {
                    return Err("Space occupied.");
                }
                positions_for_new[played.position] = played.piece;                
                return Ok(
                        TableState{
                            positions: positions_for_new,
                            player: played.piece,
                        })
            },
            None => {
                return Err("No new moves submitted.");
            }
        } 
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

pub struct GameMaster{
    win_options: WinOptions,
    game_history: Vec::<TableState>,
     
}

impl GameMaster{
    /* [x] new
     * [x] add_move
     * [x] reverse (backtrack)
     * [x] print table
     * [x] check for win
     * [x] check move legality integrated in TableState
     */

    //interface 
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
        let new_play = Some(PlaySelector::new(piece, position));
        self.game_history.push(
            self.game_history.last().unwrap().duplicate_with_new(new_play)
                .unwrap());
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
        println!("Player: {}", reference_table.player); 
         
        println!("\t {} | {} | {} ", t_positions[0], t_positions[1], t_positions[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[3], t_positions[4], t_positions[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[6], t_positions[7], t_positions[8]);
        return Ok(());
    }
}
