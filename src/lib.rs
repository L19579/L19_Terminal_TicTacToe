//See 24.
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

struct PositionSelector{
    piece: Piece,
    position: usize,
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
    
    fn duplictate_with_new(&self, previous_table: &TableState, 
                           new_play: Option<PositionSelector>) 
        -> Result<TableState, &'static str >{
        let positions_for_new: [Piece; 9];
        for i in 1..10 {
            positions_for_new[i] = previous_table.positions[i];      
        }
        match new_play {
            Some(play) => {
                if positions_for_new[play.position] != Piece::Clear
                {
                    return Err("Space occupied.");
                }
                positions_for_new[play.position] = play.piece;                
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
    pub fn new(){
        
    }
    pub fn add_move(_piece: Piece, _position: u8){
        //TODO: Check last TableStact pos, is move legal?
        //
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
