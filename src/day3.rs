use std::fs;


pub fn day3 () {
   const FNAME : &str = "./files/day3.txt" ;
// const FNAME : &str = "./files/test3.txt" ;
    let mut sum_prio: i64 = 0;
    let mut first_half_val: i64 = 0;
    let mut second_half_val: i64 = 0;
    let UPPER_A = 65;
    let lower_a = 97;

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogRocket: Should have been able to read the file");
//println!("info.txt context =\n{file_contents}");
for line in file_contents.lines() {
    println!("Current Line: {}", line);
    let mut match_char:char = '9'; 
    // 9 is not a match (null might be better)
    let str_len:usize = line.chars().count();
    if (str_len % 2 != 0) {
        println!(" THIS GUY ISN'T AN EVEN LINE ");

    }

    let half_str_len = str_len / 2;
    let part1_str = &line[..half_str_len];
    let part2_str = &line[half_str_len..];
    println!("Cut Line: {} ** {}", part1_str,part2_str);
    // now that the line is cut in half - iterate across the first looking for a match on the second
'outer_loop:    for i in 0..half_str_len {
        let utf_char:u8 = part1_str.as_bytes()[i];
        let char_char:char =utf_char as char; 
        let match_loc = part2_str.find(char_char);
        if match_loc != None {
            match_char = char_char; // will need to unset this each line (or will it get unscoped?)
            println!("matching on {}", match_char);
        }
        let char_int = match_char as i64;
        if (char_int >= lower_a) && (char_int <= lower_a+26) {
            sum_prio = sum_prio + (char_int - lower_a +1); 
            println!("match on {} for value of {}", match_char,char_int - lower_a + 1);
            // is this wrong - we found a match so stop looking is "breaking bad" :)
            println!("\n Total: {}", sum_prio);
            break 'outer_loop;
        }
        else { 
            if (char_int >= UPPER_A) && (char_int <= UPPER_A+26) {
                sum_prio = sum_prio + (char_int - UPPER_A+27); 
                println!("match on {} for value of {}", match_char,char_int - UPPER_A+27 );
                println!("\n Total: {}", sum_prio);
                break 'outer_loop;
            }
                
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
println!("\n Total: {}", sum_prio);
println!("done");

}