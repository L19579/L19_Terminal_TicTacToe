# L19_Terminal_TicTacToe
---
## Purpose
Testing Cargo.io submissions, versioning, and documentation.
---
##Game Description
###  Tests will be written around a game of TicTacToe

   |   |                        a1 |a2 |a3 \ 
---|---|---                     ---|---|---\
   |   |      --> Positions:    b1 |b2 |b3 \
---|---|---                     ---|---|---\
   |   |                        c1 |c2 |c3 
---
### External Links and Documentation.
[Placeholder #1: Crates.io] (https://www.crates.io)
[Placeholder #2: Docs,rs] (https://www.Docs.rs)
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

![Opener](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/4_start_large.png) ![Closer](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/5_end_large.png)

