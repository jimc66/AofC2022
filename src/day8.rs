use std::fs;
//use std::collections::HashMap;
// 383520 someone else
pub fn day8 () {
 /*  
    Pretty much for all days - read in the file
    */    
    const FNAME : &str = "./files/day8.txt" ;
   let width = 99;
    let height = 99;

//    const FNAME : &str = "./files/test8.txt" ;
//    let width = 5;
//    let height = 5;

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogERROR: Should have been able to read the file");
    
    let mut array_vec = vec![vec![0; width]; height];
    let range_start:usize = 0;
    let range_end:usize = width;
    let mut row:usize = 0;
    for line in file_contents.lines() {
        for looper in range_start..range_end {
        //    let looper_usize:usize = looper as usize;
            let looper_usize:usize = looper;
            
            let char_val = line.chars().nth(looper_usize).unwrap();
            let int_val =char_val.to_digit(10).unwrap();
            println! ("current int {}", int_val);
            array_vec [row][looper] = int_val;
        }
    row = row + 1;
    }
let mut vis_trees = width+width+height+height-4; //count the 4 corners one time
let mut current_see_val:u32 = 0;
for outlooper in 1..height-1 {
    /* so check the 4 directions
    assume all of the edges are good (start +1 end -1)
    maybe a position check fn that returns T or F?
    pass array x y 
    for 
    */
    for inlooper in 1..width-1 {
        let is_seen = tree_checker(&array_vec,outlooper, inlooper);
        if is_seen {
            vis_trees = vis_trees+1;
        }
        let temp_see_val = view_checker(&array_vec,outlooper, inlooper);
        if temp_see_val > current_see_val {
            current_see_val=temp_see_val;
            println!("******** MAX SEE  {}", temp_see_val);
        }
        println!("total trees seen so far {}", vis_trees);
        println!("current see val:{} max see val{}", temp_see_val,current_see_val);
    }
}

}


fn tree_checker (local_array_vec:&Vec<Vec<u32>>, row:usize, col: usize) -> bool{
    let mut clear_tree = false; // start with assume it's false - test for any block
    let check_tree_height = local_array_vec[row][col];
//    println!("we're here - row:{}  col:{} current height:{}",row,col, local_array_vec[row][col]);
    // check left
    let end_val =0;
    let start_val = col;
    'left_check: for col_val in (end_val..start_val).rev() { // rev for count down
        if check_tree_height <= local_array_vec[row][col_val]{
            clear_tree = false;
            break 'left_check; // it's blocked - exit with a false;
        }
        else {
            clear_tree = true;
        }
    }
    if clear_tree {return  clear_tree;}
    // check right
    let end_val = local_array_vec[row].len();
    let start_val = col+1;
    'right_check: for col_val in (start_val..end_val) { 
        if check_tree_height <= local_array_vec[row][col_val]{
            clear_tree = false;
            break 'right_check; // it's blocked - exit with a false;
        }
        else {
            clear_tree = true;
        }
    }
    if clear_tree {return  clear_tree;}

    // check up
    let end_val = 0;
    let start_val = row;
    'up_check: for row_val in (end_val..start_val).rev() { // rev for count down
        if check_tree_height <= local_array_vec[row_val][col]{
            clear_tree = false;
            break 'up_check; // it's blocked - exit with a false;
        }
        else {
            clear_tree = true;
        }
    }
    if clear_tree {return  clear_tree;}

    // check down
    let end_val = local_array_vec[col].len();
    let end_val = local_array_vec.len(); // IS COL RIGHT HERE???? (thinking is the lenght of the current collumn is the # or rows for that col)
//    let end_val = height; // IS COL RIGHT HERE???? (thinking is the lenght of the current collumn is the # or rows for that col)
    let start_val = row+1;
    'down_check: for row_val in (start_val..end_val) { 
        if check_tree_height <= local_array_vec[row_val][col]{
            clear_tree = false;
            break 'down_check; // it's blocked - exit with a false;
        }
        else {
            clear_tree = true;
        }
    }
    if clear_tree {return  clear_tree;}


    return clear_tree; // this should always be false
}

/* ===========    view_checker
To measure the viewing distance from a given tree, look up, down, left, and right
from that tree; stop if you reach an edge or at the first tree that is the same height 
or taller than the tree under consideration. 
(If a tree is right on the edge, at least one of its viewing distances will be zero.)

A tree's scenic score is found by multiplying together its viewing distance in each of 
the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
*/
fn view_checker (local_array_vec:&Vec<Vec<u32>>, row:usize, col: usize) -> u32 {
    let mut left_ct:u32 = 0;
    let mut right_ct:u32 = 0;
    let mut up_ct:u32 = 0;
    let mut down_ct:u32 = 0;

    let mut tallest_seen_tree:u32 = 0; // use this for each direction to know when you can't see

    let mut clear_tree = false; // start with assume it's false - test for any block
    let check_tree_height = local_array_vec[row][col];
    println!("for row:{}  col:{} current height:{}",row,col, local_array_vec[row][col]);
    // check left
    let end_val =0;
    let start_val = col;
    'left_check: for col_val in (end_val..start_val).rev() {
        // rev for count down
        let tree_to_evaluate =  local_array_vec[row][col_val];  
        if  tallest_seen_tree >= check_tree_height{
                break 'left_check; // it's blocked - exit with a false;
            }
        else {
            left_ct=left_ct+1;
            if tree_to_evaluate >= tallest_seen_tree {
                tallest_seen_tree=tree_to_evaluate
                }
            }
        }

    // check right
    let end_val = local_array_vec[row].len();
    let start_val = col + 1;
    tallest_seen_tree=0;
    'right_check: for col_val in (start_val..end_val) { 
        let tree_to_evaluate = local_array_vec[row][col_val];
        if tallest_seen_tree >= check_tree_height{
            break 'right_check; // it's blocked - exit with a false;
        }
        else {
            right_ct = right_ct+1;
            if tree_to_evaluate >= tallest_seen_tree {tallest_seen_tree=tree_to_evaluate}
        }
    }
  

    // check up
    let end_val = 0;
    let start_val = row ;
    tallest_seen_tree=0;
    'up_check: for row_val in (end_val..start_val).rev() { // rev for count down
        let tree_to_evaluate = local_array_vec[row_val][col];
//        if tallest_seen_tree >= local_array_vec[row_val][col]{
            if tallest_seen_tree >= check_tree_height{

            //            up_ct = up_ct+1;
            break 'up_check; // it's blocked - exit with a false;
        }
        else {
            up_ct = up_ct+1;
            if tree_to_evaluate >= tallest_seen_tree {tallest_seen_tree=tree_to_evaluate}
        }
    }
 

    // check down
    let end_val = local_array_vec[col].len(); // IS COL RIGHT HERE???? (thinking is the lenght of the current collumn is the # or rows for that col)
    let start_val = row+1; //guess
    tallest_seen_tree=0;
    'down_check: for row_val in (start_val..end_val) { 
        let tree_to_evaluate = local_array_vec[row_val][col];
            if tallest_seen_tree >= check_tree_height{

            //            down_ct = down_ct+1;
            break 'down_check; // it's blocked - exit with a false;
        }
        else {
            down_ct = down_ct+1;
            if tree_to_evaluate >= tallest_seen_tree {tallest_seen_tree=tree_to_evaluate}
        }
    }
    let ret_val = left_ct * right_ct * up_ct * down_ct;
    return ret_val; // this should always be false
}