use std::{fs};
//use std:str;


pub fn day3 () {
   const FNAME : &str = "./files/day3.txt" ;
// const FNAME : &str = "./files/test3.txt" ;
    let mut sum_prio: i64 = 0;
    let mut sum_part2: i64 = 0;
    let mut first_half_val: i64 = 0;
    let mut second_half_val: i64 = 0;
    let UPPER_A = 65;
    let lower_a = 97;
    let mut sum_partb:i64 = 0;
    let mut string_elements =Vec::new();

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogERROR: Should have been able to read the file");
/*
DAY 3 PART A / 1
(with a little cheat where we make the vector for part 2)
*/
for line in file_contents.lines() {
    println!("Current Line: {}", line);
    let mut match_char:char = '9'; 
    // 9 is not a match (null might be better)
    string_elements.push(line);  //creating vec for part 2
    let str_len:usize = line.chars().count();
    if str_len % 2 != 0 {
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

println!("\n Total - DAY 3 PART 1: {}", sum_prio);
}

/*
DAY 3 PART B / 2
read lines in groups of 3

*/
//let num_lines = CountLines(file_contents);

let mut total_lines:usize =0;
let input_string:&String = &file_contents.clone();
match CountLines(input_string){
    Ok(total_lines2) =>  total_lines=total_lines2 as usize,
    Err(error_val) => println!("empty file: {} ", error_val),
}
println!("vector length: {}  function return val: {}", string_elements.len(), total_lines);

// OK - ready to read through the lines
for x in (0..total_lines).step_by(3) {
        let mut return_char:char ='9';
        println!("{}", x);

        match FindCommonChar(x,3,&string_elements){
            Ok(ret_char) => return_char=ret_char ,
            Err(error_val) => println!("ERROR: {} ", error_val),
        }
        // we have a match here - so add it up - probably need to bail on the prior line if errors?
        let char_int = return_char as i64;
        if (char_int >= lower_a) && (char_int <= lower_a+26) {
            sum_part2 = sum_part2 + (char_int - lower_a +1); 
            println!("match on {} for value of {}", return_char,char_int - lower_a + 1);
            // is this wrong - we found a match so stop looking is "breaking bad" :)
            println!("\n Total: {}", sum_part2);
        //    break 'outer_loop; /// DON'T THINK I NEED THIS
        }
        else { 
            if (char_int >= UPPER_A) && (char_int <= UPPER_A+26) {
                sum_part2 = sum_part2 + (char_int - UPPER_A+27); 
                println!("match on {} for value of {}", return_char,char_int - UPPER_A+27 );
                println!("\n Total: {}", sum_part2);
        //        break 'outer_loop;  // DPN"T NEED THIS
            }
        
    }
}
//println!("number of lines {}", total_lines);
println!("\n Total - DAY 3 PART 1: {}", sum_prio);
println!("\nPart 2 Total: {}", sum_part2);
println!("done");

}


/*
CountLines
really to learn how to use a function that takes  / returns a value
 */
fn CountLines(inpstr:&str) -> Result<i32, String>
{
    let mut cnt  = 0;
    for line in inpstr.lines() {
        cnt = cnt + 1;
    }
    if cnt == 0 {
        panic!("No LINES IN THIS OLE STRING");
    }
    Ok( cnt )
}
 

/********************************************************************
*   FindCommomnChar
*   really to learn how to use a function that takes  / returns a value
*********************************************************************/

fn FindCommonChar(start_val:usize, number_to_check:usize, invec:&Vec<&str>) -> Result<char, String>
{
    if number_to_check != 3 {
        panic!("SORRY - HARD CODED FOR 3 ITEMS ");
    }
    let mut ret_char  = '9' as char;
    let bad_char = '8' as char;
    let vector_len = invec.len();
    let starter_usize:usize = start_val;
    let end_usize:usize = starter_usize + number_to_check;
    let mut match_char:char = '9'; //start with a bogus character to match , prob dup with ret_char?

    println!("start: {} end:{}",start_val,end_usize);
    let str_len:usize = invec[start_val].chars().count();
    'first_iter_loop: for i in 0..str_len { //go through all the chars in string 1 as the searcher
        let utf_char:u8 = invec[start_val].as_bytes()[i];
        let char_char:char =utf_char as char; 
        let match_loc = invec[start_val+1].find(char_char); // see if you find this char in 2nd string
        if match_loc != None { //2nd line has a match
            let match_loc2 = invec[start_val+2].find(char_char); // see if you find this char in 2nd string
            if match_loc2 != None { //3rd line has a match!!
            match_char = char_char; 
            println!("matching on {}", match_char);
            break 'first_iter_loop;
            }
        } 

    }
/* 
    for iterator in starter_usize..end_usize {
        println!("vector {}", invec[iterator]);
        let str_len:usize = invec[iterator].chars().count();
        println!("string len {}", str_len);
        
        


    }*/
    ret_char = match_char;
    if ret_char == bad_char {
        panic!("WHATTTT - NO MATCH");
    }
    Ok( ret_char as char )
}