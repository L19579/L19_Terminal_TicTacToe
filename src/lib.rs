//See 24.
use std::fmt;
use std::marker::Copy;

pub enum Piece{
    User,
    Npc,
    Clear,
}

struct TableState{
    pub positions: [Piece; 9],
}

impl TableState{
    fn new() -> TableState {
        return TableState{
            positions: [Piece::Clear; 9]  
        };
    }
}

impl Copy for TableState{
    // Getting implementation error for copy. No "explicit" ptrs
    // in this struct. Focus on this first.
}

pub struct TicTocToe{
    gameplay_history: Vec::<TableState>, 
}

impl TicTocToe{
    //interface 
    pub fn new(){
        //
    }
    pub fn add_move(_piece: Piece, _position: u8){
        //TODO: Check last TableStact pos, is move legal?
        //
    } 

    pub fn backtrack(_jumps: u8){
        let jumps = _jumps;
        if jumps < 2 { 
            jumps = 2; 
        }
    }
    
}

impl fmt::Display for TableState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
    }
}
