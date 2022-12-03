use std::fs;

pub fn day2 () {
   const FNAME : &str = "./files/day2.txt" ;
// const FNAME : &str = "./files/sample2.txt" ;
    let mut player1_score: i64 = 0;
    let mut player2_score: i64 = 0;
    let mut player2_new_score: i64 = 0;

    // x rock y paper z scissor
    // game combos - sorted by player1 win, draw, player 2 win
    // player one wins 
    // rock - scissor (A Z); paper - rock (B X); scissor paper (C Y)
    // tie -rock rock etc (A X; B Y; C Z)
    // player 2 rock - paper (A Y); paper - scissor (B Z); scissor - rock (C X)
    let game_combos  = [
        "A Z", "B X", "C Y",
        "A X", "B Y", "C Z",
        "A Y", "B Z", "C X"
        ]; 
    let player1_scores = [
        7, 8, 9,
        4, 5, 6,
        1, 2, 3
        ];
    let player2_scores = [
        3, 1, 2,
        4, 5, 6,
        8, 9, 7
        ];
    // X lose, Y Draw, Z win
    let player2_new_scores = [
            8, 1, 6,
            3, 5, 7,
            4, 9, 2
            ];
            
    let mut winning_val = -1;

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogRocket: Should have been able to read the file");
//println!("info.txt context =\n{file_contents}");
for line in file_contents.lines() {
    println!("{}", line);
    /*  // found a better way
    let player1 = line.as_bytes()[0];
    let player2 = line.as_bytes()[2];
    */
      //println!("as a num {}",i );
    winning_val = -1; 
    let index_val = game_combos
        .iter()
        .position(|&x| x == line)
        .unwrap();
    println!("location_match {}", index_val);
    player1_score = player1_score+player1_scores[index_val];
    player2_score = player2_score+player2_scores[index_val];
    player2_new_score = player2_new_score+player2_new_scores[index_val];
    

      }

println!("\n player1 score with part 1:  {}", player1_score);
println!("\n player2 score with part 1:  {}", player2_score);
println!("\n ===    Now Part 2 \n");
//let tot = max2_num+max3_num+max_num;
//println!("top 3 tot {}", tot);
println!("\n player2 score with part 2:  {}", player2_new_score);
println!("done");

}