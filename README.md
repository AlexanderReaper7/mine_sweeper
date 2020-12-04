# mine_sweeper
This is a hobby project with the purpose of learning the Rust programming language.

## Instructions
Run the .exe with three arguments, in the following order and type:   Columns:usize Rows:usize MineConcentration:f64.
### Example
`mine_sweeper.exe 60 30 0.15`<br/>
This creates a game with 60 columns and 30 rows where roughly 15% of the cells are mines.<br/>
<br/>
Left click to reveal a cell.<br/>
Right click to flag a cell.<br/>
You win when all mines have been flagged, victory is shown by the screen becoming green.<br/>
You lose by revealing a mine, this is shown by a red screen.<br/>
<br/>
## TODO
- Add instructions for what the colors mean.<br/>
- Implement AI versions.<br/>
