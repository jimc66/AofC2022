use std::fs;
use std::thread;
use std::io::Write;
use std::io::stdout;

/*use std::str::Split;    
use std:num;
use std::collections::HashMap;
*/

pub fn day5 () {
   /*  
    Pretty much for all days - read in the file
    */    
    const FNAME : &str = "./files/day5.txt" ;
    //const FNAME : &str = "./files/test5.txt" ;

       let file_contents = fs::read_to_string(FNAME)
       .expect("LogERROR: Should have been able to read the file");
   /*
   DAY 5 PART A / 1 */
   println!("DAY 5 Starting");
   let mut move_vec:Vec<Vec<usize>> = vec![vec![]]; // for the move instructions
   let mut current_move:usize = 0;

// take the modified file with the move info and add the values to a vector (# items, from, to)
   for line in file_contents.lines() {
    let line_parts2 = line.replace("move ", "");
    let line_parts3 = line_parts2.replace("from ", "");
    let line_parts_final = line_parts3.replace("to ", "");
    // line_parts_final has 3 numbers as string separated by spaces
    let line_nums = line_parts_final.split_whitespace();
    let mut line_part_iter:usize=0;
    let mut the_three_nums:Vec<usize> = vec![];
    for line_part in line_nums {
        println!("{}", line_part);
        let line_part_usize= line_part.parse::<usize>().unwrap(); // does this make a number?
        let vec_len = move_vec.len();
    //    move_vec[vec_len].push(line_part_usize);
        the_three_nums.push(line_part_usize);
        line_part_iter=line_part_iter + 1;
    }
    move_vec.push(the_three_nums.clone()); // add a blank one
    current_move = current_move +1;
    the_three_nums.clear();
   }
/*
now that we have the move instructions, initialize the cargo
load_vec2 is for the test file
load_vec3 is for the real file
 */
    let mut testvec = vec![vec!['#'; 80]; 24];
    let mut test_cargo_vec = vec![vec![];3];
    let mut cargo_vec = vec![vec![];9];

    load_vec(&mut testvec);
    load_vec2(&mut test_cargo_vec);
    load_vec3(&mut cargo_vec);

let char_check = cargo_vec[0][0];
println!("cargo check - fist item first row {} ", char_check);
let len_cargo_vec = cargo_vec.len();
let last_item_depth = cargo_vec[len_cargo_vec-1].len();
let char_check2 = cargo_vec[len_cargo_vec-1][last_item_depth-1];
println!("cargo check - last item last row {} ", char_check2);
/* 
now do the actual moving
 */

//move_cargo(&mut cargo_vec, move_vec); //real for part 1
move_cargo2(&mut cargo_vec, move_vec); //real
//move_cargo(&mut test_cargo_vec, move_vec); //test
// print out the top of the stacks
print!("TOP OF STACKS1: ");
let cargo_size = cargo_vec.len();  // to switch between test and real
//let cargo_size = test_cargo_vec.len();
for i in 0..cargo_size{
    let top_item_pos = cargo_vec[i].len();
//    let top_item = cargo_vec[i][top_item_pos];
    let top_item_pos = cargo_vec[i].len()-1;
    let top_item = cargo_vec[i][top_item_pos];

    print!("{}",top_item);
}
stdout().flush();
println!("\ndone");

}

/*
move_cargo2 
DAY 2
for doing the actual cargo move
*/
fn move_cargo2 (cargo_vector: &mut Vec<Vec<char>>, move_vector:Vec<Vec<usize>> ){
    // a little pre-flight check - not sure if we need it
        let len_move_vector = move_vector.len();
        let len_cargo_vec = cargo_vector.len();
        let last_item_depth = cargo_vector[len_cargo_vec-1].len();
        println!("checking last item: {}", cargo_vector[len_cargo_vec-1][last_item_depth-1]);
        let move_iterator = move_vector.iter();
        for instruction in 1..len_move_vector {
            let items_to_move = move_vector[instruction][0];
            let from_row = move_vector[instruction][1]-1; // get to 0 start
            let to_row = move_vector[instruction][2]-1; // 0 start
            println!("moving {} from {} to {} (base 0)", items_to_move, from_row, to_row);
            if items_to_move == 1{
                for move_item in 0..items_to_move{
                    let item_to_move = cargo_vector[from_row].pop().unwrap();
                    cargo_vector[to_row].push(item_to_move);
                }
            }
            else { // we need the move setup
                let split_loc = cargo_vector[from_row].len() - items_to_move;
                let new_vec = cargo_vector[from_row].split_off(split_loc);
                for i in 0..items_to_move{
                    cargo_vector[to_row].push(new_vec[i]);
                }
            }
        }
        
    }

/*
move_cargo
for doing the actual cargo move
*/
fn move_cargo (cargo_vector: &mut Vec<Vec<char>>, move_vector:Vec<Vec<usize>> ){
// a little pre-flight check - not sure if we need it
    let len_move_vector = move_vector.len();
    let len_cargo_vec = cargo_vector.len();
    let last_item_depth = cargo_vector[len_cargo_vec-1].len();
    println!("checking last item: {}", cargo_vector[len_cargo_vec-1][last_item_depth-1]);
    let move_iterator = move_vector.iter();
    for instruction in 1..len_move_vector {
        let items_to_move = move_vector[instruction][0];
        let from_row = move_vector[instruction][1]-1; // get to 0 start
        let to_row = move_vector[instruction][2]-1; // 0 start
        println!("moving {} from {} to {} (base 0)", items_to_move, from_row, to_row);
        for move_item in 0..items_to_move{
            let item_to_move = cargo_vector[from_row].pop().unwrap();
            cargo_vector[to_row].push(item_to_move);
        }
    }
    
}


/* for loading up the real cargo */
fn load_vec3(loadvec: &mut Vec<Vec<char>>){
    loadvec[0].push('S');
    loadvec[0].push('L');
    loadvec[0].push('W');

    loadvec[1].push('J');
    loadvec[1].push('T');
    loadvec[1].push('N');
    loadvec[1].push('Q');

    loadvec[2].push('S');
    loadvec[2].push('C');
    loadvec[2].push('H');
    loadvec[2].push('F');
    loadvec[2].push('J');

    loadvec[3].push('T');
    loadvec[3].push('R');
    loadvec[3].push('M');
    loadvec[3].push('W');
    loadvec[3].push('N');
    loadvec[3].push('G');
    loadvec[3].push('B');

    loadvec[4].push('T');
    loadvec[4].push('R');
    loadvec[4].push('L');
    loadvec[4].push('S');
    loadvec[4].push('D');
    loadvec[4].push('H');
    loadvec[4].push('Q');
    loadvec[4].push('B');

    loadvec[5].push('M');
    loadvec[5].push('J');
    loadvec[5].push('B');
    loadvec[5].push('V');
    loadvec[5].push('F');
    loadvec[5].push('H');
    loadvec[5].push('R');
    loadvec[5].push('L');
      
    loadvec[6].push('D');
    loadvec[6].push('W');
    loadvec[6].push('R');
    loadvec[6].push('N');
    loadvec[6].push('J');
    loadvec[6].push('M');

    loadvec[7].push('B');
    loadvec[7].push('Z');
    loadvec[7].push('T');
    loadvec[7].push('F');
    loadvec[7].push('H');
    loadvec[7].push('N');
    loadvec[7].push('D');
    loadvec[7].push('J');

    loadvec[8].push('H');
    loadvec[8].push('L');
    loadvec[8].push('Q');
    loadvec[8].push('N');
    loadvec[8].push('B');
    loadvec[8].push('F');
    loadvec[8].push('T');

 /* input data - prob about as long as parsing in the file, so I typed it!
     [B][L][J]
    [B][Q][R][D][T]
    [G][H][H][M][N][F]
    [J][N][D][F][J][H][B]
    [Q][F][W][S][V][N][F][N]
    [W][N][H][M][L][B][R][T][Q]
    [L][T][C][R][R][J][W][Z][L]
    [S][J][S][T][T][M][D][B][H]
*/


}

fn load_vec2(loadvec: &mut Vec<Vec<char>>){
    loadvec[0].push('Z');
    loadvec[0].push('N');

    loadvec[1].push('M');
    loadvec[1].push('C');
    loadvec[1].push('D');
    
    loadvec[2].push('P');
    
}



fn load_vec(vec: &mut Vec<Vec<char>>){
    vec[0][0] = 'd';
    
    vec[23][79] = 'd';
}

