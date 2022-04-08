//! # L19_Terminal_TicTacToe
//!
//! `L19_Terminal_TicTacToe` is simple game of Tic Tac Toe made to run
//! a command line. It was created as an educacutional exercise, and to
//! improve the creator's ability to document and log changes on various
//! local, and remote platforms.
//!  
//! Examples are NOT Doc-test enabled.
pub mod key_bindings;
use rand::{ Rng, thread_rng };
use std::{
    io::{Write, stdin,},
    cmp::PartialEq,
    collections::HashMap,
    time::Duration,
    thread, //heavy
    fmt, 
};

/// Enum acts as player marker on the table.
pub enum Piece{
    User,
    Npc,
    Clear,
}

impl Piece{
    /// Return Piece representation as ``&'static str`
    ///
    /// # Example
    /// 
    /// ```
    /// let player_one = Piece::Npc;
    /// let player_one_str: &str= player_one.as_str();
    /// assert_eq!("Npc", player_one);
    /// ```
    pub fn as_str(&self) -> &'static str {
        return match *self{
                Piece::User => "User",   
                Piece::Npc => "Npc",    
                Piece::Clear => "Clear",    
        } 
    }
    /// Returns the opposite of given piece.
    /// 
    /// #Panics 
    /// 
    /// Panics if Pie::Clear is passed as an argument.
    ///
    /// # Example
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let peice : Piece = Piece::User;
    /// let opp_piece = peice.opposite();
    ///     
    /// let opp_as_str: &str = opp_piece.as_str();
    /// assert_eq!("Npc", opp_as_str);
    /// ```
    pub fn opposite(&self) -> Piece {
        match *self {
            Piece::User => Piece::Npc,
            Piece::Npc => Piece::User,
            _ => panic!("Invalid input."),
        } 
    }

    /// Allows return of `Peice` variant using u8 as an argument.
    /// An argument that isn't 0, or 1 returns Piece::Clear.
    /// 
    /// # Example
    ///
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let a_piece: Piece = select_with_int(1);
    /// let a_piece_as_str: &str = a_piece.as_str();
    ///
    /// assert_eq!("Npc", a_piece_as_str);
    /// ```
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
    /// Eq/PartialEq trait implementations for `Piece` enum.
    fn eq(&self, other: &Piece) -> bool{
        //Easy to make the obv error here. Tracking runtime
        //bugs on a large codebase, not so much.
        return self.as_str() == other.as_str();
    }
}

impl Copy for Piece{}
impl Clone for Piece{
    /// Copy/Clone trait implementations for `Piece` enum.
    fn clone(&self) -> Piece {
        return *self;
    }
}


impl fmt::Display for Piece{
    /// Display trait implementation for `Piece`. 
    /// Returns "X", "O", or " " to formatter.
    /// Used only to print icons on table. 
    /// See `as_str()` in Piece for full string representation
    /// of variant.
    /// 
    /// # Example
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let user_piece: Piece = Piece::User;
    /// 
    /// println!("Symbol for user is: {}", user_piece);
    /// ```
    fn fmt(&self, formatter: &mut fmt::Formatter)
        -> fmt::Result{
            return match *self{
                Piece::User => write!(formatter, "X"),    
                Piece::Npc => write!(formatter, "O"),    
                Piece::Clear => write!(formatter, " "),    
            };
        }
    
}

/// Interface for structured submissions to TableState.duplicate_with_new()
struct PlaySelector{
    pub piece: Piece,
    pub position: usize, 
    //usize to avoid numerous cast calls for indexing.
    //u8 preferable.
}

impl PlaySelector{
    /// Returns a new `PlaySelector` object.
    fn new (piece: Piece, position: usize) -> PlaySelector {
        return PlaySelector {
            piece,
            position,
        };
    }
}
/// Data struct; holds valid winning table allignments for a
/// `Piece` variants.
struct WinOptions{
    options: [(usize, usize, usize); 8], 
}

impl WinOptions{
    /// Returns a new WinOptions object.
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
    /// Returns array of all winning position allignments.
    /// # Example 
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let w_pos = WinOptions::new();
    /// 
    /// println!("Winning positions are {:?}", w_pos.options());
    /// ```
    fn options(&self) -> [(usize, usize, usize); 8] {
        return self.options;
    }
}

/// Designed to be chained in order to track and perpetuate game
/// state changes. 
struct TableState{
    positions: [Piece; 9],
    player: Piece,
}

impl TableState{
    /// Returns a new TableState object.
    /// `positions` and `player` are set to hold `Piece::Clear`
    /// to denote empty table.
    /// This is typically called once to initiate chain of
    /// `TableState` objects that track the game's progression.
    fn new() -> TableState {
        return TableState{
            positions: [Piece::Clear; 9],
            player: Piece::Clear,
        };
    }
    
    /// Returns reference to array representing `Piece` variant
    /// positions on board.
    fn positions(&self) -> &[Piece; 9]{
        return &self.positions;
    }
    
    /// Returns reference to `Piece` representing variant last
    /// played on board.
    fn player(&self) -> &Piece{
        return &self.player
    }
    /// Returns a new `TableState` holding last table state's 
    /// values plus one new change. Returned object is expeceted
    /// to be stored in a statically ordered collection representing
    /// game state transitions. 
    ///
    /// Expects `PlaySelector` object as argument.
    /// Returns `Result::Err(e)` if position insert is occupied by
    /// a non `Piece::Clear` variant.
    ///
    /// # Example 
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let game_progression = Vec::from(TableState::new());
    /// let new_move = PlaySelector::new(Piece::User, 1);
    /// game_progression.push(new_move);
    ///
    /// let last_piece_as_str: &str = 
    ///     game_progression.last().position[1].as_str;
    /// assert_eq!(last_piece_as_str, "User");
    /// ```
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
    /// Returns `true` if non Piece::Clear variants on the table
    /// matchs a `WinOptions.options` tuple. This indicates that either
    /// `Piece::User`, or `Piece::Npc` holds a winning allignment.
    ///
    /// # Example
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let game_progression = Vec::from(TableState::new());
    /// let new_move = PlaySelector::new(Piece::User, 3);
    /// game_progression.push(TableState::duplicate_with_new(new_move));
    /// let is_win = game_progression.last().is_win();
    ///
    /// assert_eq!(is_win, false);
    /// ```
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
    /// Copy/Clone trait implementations for `TableState`.
    fn clone(&self) -> TableState{
        return *self;
    }
}

/// Game controller and primary interface to main.rs
pub struct GameMaster <'a>{
    under_score: &'a str,
    win_options: WinOptions,
    game_history: Vec::<TableState>,
    key_bindings: &'a HashMap<&'a str, usize>,
}

impl<'a> GameMaster<'a>{
    /// Returns time bound `GameMaster` object.
    /// This offers main.rs an opportunity to set custom `key_bindings`.
    /// See main.rs for default bindings.
    ///
    ///  `under_score` holds "_"; needed for repeated prints.
    ///  `win_options` holds winning allignment options.
    ///  `game_history` tracks table state progression.
    ///  `key_bindings` stores input binding data. 
    pub fn new(key_bindings_c: &'a HashMap<&'a str, usize>) -> GameMaster<'a> {
        let under_score: &str = "_";
        let win_options = WinOptions::new();
        let mut game_history = Vec::<TableState>::new();
        game_history.push(TableState::new());
        
        return GameMaster{
            under_score,
            win_options,
            game_history,
            key_bindings: key_bindings_c,
        }
    }
    
    /// Automates: User input fetch, Npc plays, legality checks, and `TableState`
    /// terminal visualization.
    /// This can only be called if `game_history.last().player()` holds a non
    /// Piece::Clear variant.
    ///
    /// # Example
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let bindings = key_bindings::default_bindings();
    /// let gm = GameMaster::new(&bindings);
    /// gm.add_move(Piece::Npc, 2);
    /// gm.next_mover_w_prompt(Piece::Npc);
    /// 
    /// let last_piece : &str = gm.game_history.last().player().as_str();
    /// assert_eq!(gm.game_history.len(), 3);
    /// assert_eq!(last_piece, "User"); 
    /// ``` 
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
                print!("Your turn: ");
                std::io::stdout().flush().unwrap();
                let mut user_input = String::new();
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
    
    /// This function returns a new table state when called for move submission with
    /// any `Piece` variant.
    /// `Result::Err(e)` is returned if position of interest is held by a non `Peice;:Clear`
    /// variant.
    /// 
    /// # Example
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let bindings = key_bindings::default_bindings();
    /// let gm = GameMaster::new(&bindings);
    /// gm.add_move(Piece::Npc, 5);
    /// 
    /// let npc_as_str = gm.game_history.last().player().as_str();
    /// assert_eq!(npc_as_str, "Npc");
    /// ```
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

    /// Initiates random legal move for Npc via `add_move()`. 
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
   
    /// Allows user to jump to a previous game state.
    /// Returns `Result::Err(e)` if rangle is beyond `game_history.len()`
    /// Always returns to game state awaiting user's input. 
    /// 
    /// This function is NOT in use and hasn't been tested. It will
    /// see updates on next release. 
    pub fn back_track(&mut self, jumps: u8) -> Result<(), &'static str>{
        if jumps < 1 as u8 ||  
            jumps*2 >= (self.game_history.len() - 2) as u8 {
            return Err("Backtrack: out of range request.");
        }
        
        self.game_history.truncate((jumps*2)as usize); 
        println!("Jumping back to play #{}", self.game_history.len());
        return Ok(());
    }
    
    /// This function is an extension of `TableState.is_win()`. The primary
    /// difference being that it's aware of game state rather than just
    /// table state. 
    /// Returns `true` and winning non `Piece::Clear` variant if win is detected 
    /// on last play.
    pub fn check_win(&self) -> (bool, &Piece) {
        return (self.game_history.last().unwrap().is_win(&self.win_options),
            self.game_history.last().unwrap().player());
    }
    
    /// Prints a visualization of an indexed TableState in game_history on command line.
    /// Indexing is done in reverse. `game_history.last()` == 0.
    /// 
    /// Returns `Result::Err`(e) if `TableState` requested isn't within range.
    ///
    /// # Example 
    /// 
    /// ```
    /// use l19_terminal_tictactoe;
    ///
    /// let bindings = key_bindings::default_bindings();
    /// let gm = GameMaster::new(&bindings);
    /// gm.add_move(Piece::Npc, 1);
    /// gm.add_move(Piece::User, 4);
    /// gm.print_table();
    /// ```
    pub fn print_table(&self, reversed_history_index: isize) -> Result<(), &'static str>{
        let history_index = (self.game_history.len() as isize - reversed_history_index) - 1;
        if history_index < 0 {
            return Err("Request beyond acceptable range.");
        }
        let reference_table = &self.game_history[history_index as usize];
        let t_positions = reference_table.positions();
        let l = self.under_score; 
        println!("\n\t {} | {} | {} ", t_positions[0], t_positions[1], t_positions[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} ", t_positions[3], t_positions[4], t_positions[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {} | {} | {} \n", t_positions[6], t_positions[7], t_positions[8]);
        return Ok(());
    }
    
    /// Prints visualization of key binding positions on command line.
    pub fn show_labeled_table(&self){
        let l = self.under_score;
        let mut key : Vec::<&str> = Vec::new(); 
        for (k, _) in self.key_bindings.iter(){
            key.push(k);
        }
        key.sort();
        //Potential bug if user's allowed unrestricted liberties with bindings.
        //This doesn't work if the UTF-8s aren't chronological.
        println!("\n\t {}| {}| {} ", key[0], key[1], key[2]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {}| {}| {} ", key[3], key[4], key[5]);
        println!("\t{:_>3}|{:_>3}|{:_>3}", l, l, l);
        println!("\t {}| {}| {} \n", key[6], key[7], key[8]);
    
    }
}
