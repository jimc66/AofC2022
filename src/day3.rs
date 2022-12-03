use std::fs;
use substring::Substring;

pub fn day3 () {
//   const FNAME : &str = "./files/day2.txt" ;
 const FNAME : &str = "./files/test3.txt" ;
    let mut sum_prio: i64 = 0;
    let mut first_half_val: i64 = 0;
    let mut second_half_val: i64 = 0;



    let file_contents = fs::read_to_string(FNAME)
    .expect("LogRocket: Should have been able to read the file");
//println!("info.txt context =\n{file_contents}");
for line in file_contents.lines() {
    println!("{}", line);
    let str_len = line.chars().count();
    let part1_str = line.substring(0,str_len/2);
    let part2_str = line.substring(str_len/2,strlen);
    for i in 0..str_len/2 {
        let match_loc = part2_str.find(part1_str[i]);
        if match_loc >= 0 {
            let match_char = part1_str[i];
        }
    }

    /*  // found a better way
    let player1 = line.as_bytes()[0];
    let player2 = line.as_bytes()[2];
    */
      //println!("as a num {}",i );
    /* 
      winning_val = -1; 
    let index_val = game_combos
        .iter()
        .position(|&x| x == line)
        .unwrap();
    println!("location_match {}", index_val);
    player1_score = player1_score+player1_scores[index_val];
    player2_score = player2_score+player2_scores[index_val];
    player2_new_score = player2_new_score+player2_new_scores[index_val];
    */

      }

//println!("\n player1 score with part 1:  {}", player1_score);
//println!("\n player2 score with part 1:  {}", player2_score);
//println!("\n ===    Now Part 2 \n");
//let tot = max2_num+max3_num+max_num;
//println!("top 3 tot {}", tot);
//println!("\n player2 score with part 2:  {}", player2_new_score);
println!("done");

}