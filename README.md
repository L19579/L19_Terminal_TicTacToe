# L19_Terminal_TicTacToe

## Purpose
- Testing Cargo.io submissions, versioning, and documentation.
- Tests will be written around a game of TicTacToe.
---

## Game Description
#### Environment 
  Linux, x86_64.
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

- Type in input when prompted. This "bot" is just a pseudorandom number generator. It takes effort to lose.
<div align="center">
![Opener](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/4_start_large.png) ![Closer](https://raw.githubusercontent.com/L19579/L19_Terminal_TicTacToe/main/imgs/5_end_large.png)
</div>

---

## Documentation
### External Links and Documentation.
- [Placeholder #1: Crates.io] (https://www.crates.io)
- [Placeholder #2: Docs,rs] (https://www.Docs.rs)

---

## TODO
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
