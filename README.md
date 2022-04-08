# L19_Terminal_TicTacToe
---
## Purpose
- Testing Cargo.io submissions, versioning, and documentation.
- Tests will be written around a game of TicTacToe
---
##Game Description
- Test environment: Linux, x86\_64.
#### Instructions
##### Install
```bash
  git clone https://github.com/L19579/L19_Terminal_TicTacToe.git # Download repo
  cd #insert_repo_directory
  cargo build --release
```
##### Run
```bash
  ./target/release/l19_terminal_tictactoe
```

##### Controls 
- Game input bindings are set in main.rs. System will adapt to UTF-8 binding
  inputs 2 chars in length listed chronologically. 
```rust
  use l19_terminal_tictactoe as l19;
  
  //-- snip
  
  let input_bindings: Hashmap::<&str, usize> = HashMap::from([
      ("a1", 0),
      ("a2", 1),
      ("a3", 2),
      ("b1", 3),
      ("b2", 4),
      ("b3", 5),
      ("c1", 6),
      ("c2", 7),
      ("c3", 8),

  ]);

  let mut gm = l19::GameMaster::new(&input_bindings);
  
  //-- snip
```
- Type in input when prompted. This "bot" is just a pseudo random number generator. It takes effort to lose.

![Opener](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/4_start_large.png) ![Closer](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/5_end_large.png)

---
### External Links and Documentation.
- [Placeholder #1: Crates.io] (https://www.crates.io)
- [Placeholder #2: Docs,rs] (https://www.Docs.rs)

## Documentation
---
###  Initial targets
- [x] Randomizer decides which player starts.
- [x] X is assigned to player 1, O to player 2.
- [x] Turn based, updated board is printed below. Terminal isn't cleared for simplicity.
- [x] Player is presented with cell ID and types it in to select position when prompted.
- [x] X / O are drawn when player/AI make their move.
- [x] Table state is saved with every move.
- [x] System checks for wins after positions are submitted.
- [ ] Full code documentation on Docs.rs
- [ ] Top level description on Github; Listed as release.
- [ ] Top level description on Crates.io; Listed as release.

---
md formatting tests; to be removed.

#### Inline code block: 

##### Example Code
```rust
  use std::HashMap;

  fn search <'a>(request: &'a HashMap<&'a str, usize>){
    //--snip. 
  }

  fn main(){
    let name_age = HashMap::from([
      ("Aksana", 87),
      ("Devon", 58),
      ("Cronos", 42)
    ]);

    search(name_age);
  }
```

##### Build and run binary.
```bash
  cargo build --release #Build binary.
  ./target/release/my_program #Run binary
```

---
#### Images


