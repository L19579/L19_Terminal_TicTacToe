use std::fmt;
use std::cmp::PartialEq;

pub enum Piece{
    User,
    Npc,
    Clear,
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

struct PlaySelector{
    piece: Piece,
    position: usize, 
    //prefer u8 here but working around small constraint.
    //This echoes elsewhere though. Careful not to repeat
    //in the future.
}

impl PlaySelector{
    fn new (piece: Piece, position: usize) -> PlaySelector {
        return PlaySelector {
            piece,
            position,
        };
    }
}


struct TableState{
    positions: [Piece; 9],
}

impl TableState{
    fn new() -> TableState {
        return TableState{
            positions: [Piece::Clear; 9]  
        };
    }

    fn positions(&self) -> &[Piece; 9]{
        return &self.positions;
    }
    
    fn duplictate_with_new(&self, 
                        new_play: Option<PlaySelector>) 
        -> Result<TableState, &'static str >{
        let positions_for_new: [Piece; 9];
        for i in 1..10 {
            positions_for_new[i] = self.positions[i];      
        }
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
                        })
            },
            None => {
                return Ok(
                    TableState{
                        positions: positions_for_new,
                    })
            }
        } 
    }
}

pub struct GameMaster{
    game_history: Vec::<TableState>, 
}

impl GameMaster{
    //interface 
    pub fn new() -> GameMaster {
        let mut game_history: Vec::<TableState>;
        game_history.push(TableState::new());
        
        return GameMaster{
            game_history, 
        }
    }

    pub fn add_move(&self, piece: Piece, position: usize){
        let new_play = Some(PlaySelector::new(piece, position));
        self.game_history.push(
            self.game_history.last().duplicate_with_new(new_play)
                .unwrap());
    } 

    pub fn backtrack(&self, _jumps: u8) -> Result<(), &'static str>{
        let jumps = _jumps *2 ;
        if jumps < 2 { 
            jumps = 2; 
        }
        
        if self.game_history.len() => jumps {
            
            return Ok()
        } else {
            return Err("Excess range.");
        } 
    }
    
}

impl fmt::Display for TableState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
    }
}
