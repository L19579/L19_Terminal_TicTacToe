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
    
    fn duplicate_with_new(&self, new_play: Option<PlaySelector>) 
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

    fn is_win(&self, win_options: &WinOptions) -> bool{
        let win: bool = false;
        for win_option in win_options.options(){
            if self.positions[win_option.0] != Piece::Clear
                && self.positions[win_option.0]
                == self.positions[win_option.1]
                && self.positions[win_option.1]
                == self.positions[win_option.2]{
                win = true;
            }
        }
        return win;
    }
}

pub struct GameMaster{
    win_options: WinOptions,
    game_history: Vec::<TableState>,
}

impl GameMaster{
    /* [x] new
     * [x] add_move
     * [/] reverse (backtrack)
     * [ ] print board
     * [/] check for win
     * [x] check move legality integrated in TableState
     * [ ] Control flow (gameplay) - likely in main
     */

    //interface 
    pub fn new() -> GameMaster {
        let win_options = WinOptions::new();
        let mut game_history: Vec::<TableState>;
        game_history.push(TableState::new());
        
        return GameMaster{
            win_options,
            game_history, 
        }
    }

    pub fn add_move(&self, piece: Piece, position: usize){
        let new_play = Some(PlaySelector::new(piece, position));
        self.game_history.push(
            self.game_history.last().unwrap().duplicate_with_new(new_play)
                .unwrap());
    } 

    pub fn backtrack(&self, _jumps: u8) -> Result<(), &'static str>{
        let jumps = _jumps *2 ;
        if jumps < 2 { 
            jumps = 2; 
        }
        
        if self.game_history.len() >= jumps {
            // TODO --------------------------------------------------------
            return Ok()
        } else {
            return Err("Excess range.");
        } 
    }

    pub fn check_win(&self) -> bool {
        return self.game_history.last().unwrap().is_win(&self.win_options);
    }
    
}

impl fmt::Display for TableState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO --------------------------------------------------------
    }
}
