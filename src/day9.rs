use std::hash::Hash;
use std::{fs, num};
use std::collections::HashMap;
//use std::collections::HashMap;
// 383520 someone else
/*
directions
After simulating the rope, you can count up all of the positions the tail
visited at least once. In this diagram, s again marks the starting position 
(which the tail also visited) and # marks other positions the tail visited

the head (H) and tail (T) must always be touching (diagonally adjacent and 
even overlapping both count as touching):
If the head is ever two steps directly up, down, left, or right from the tail, 
the tail must also move one step in that direction so it remains close enough

Otherwise, if the head and tail aren't touching and aren't in the same row or column, 
the tail always moves one step diagonally to keep up

approach
instructions 
instructions array (or vec?) 
[0] direction
[1] number 

tail move vec ? hashmap 
Key  [height]-[width] True

head current pos 
struct [x-y]

tail current pos 
struct [x-y]

move head - read instr and move
move tail - check head pos, if out of range move (update struct) AND add to hashmap
(day 7 hashmap has the function to copy for this)
*/

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash,Clone)]
pub struct Coordinates {x:i32,y:i32}
pub struct Instructions {direction:char, spaces:i32 }

pub fn day9 () {
/*  
    Pretty much for all days - read in the file
    */    
//    const FNAME : &str = "./files/day9.txt" ;
//    const number_instructions:usize = 2000;
//   let width = 99;
//    let height = 99;

    const FNAME : &str = "./files/test9.txt" ;
    const number_instructions:usize = 8;
//    let height = 5;

    let mut head_loc:Coordinates = Coordinates { x: 0, y: 0 };
    let mut tail_loc:Coordinates = Coordinates { x: 0, y: 0 };

    let mut vec_instructions:Vec<Instructions> = Vec::new();
    // the ones I'm messing with
    let mut hash_tail:HashMap<Coordinates, i32> = HashMap::new(); 
    let mut hash_tail2:HashMap<String,i32> = HashMap::new();

    let mut vec_tail:Vec<Coordinates> =Vec::new(); 

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogERROR: Should have been able to read the file");
    for line in file_contents.lines() {
        let splits:Vec<&str>  = line.split_whitespace().collect();
        let one_direction:char = splits[0].chars().nth(0).unwrap();
        let num_spaces:i32 = splits[1].parse().unwrap();
        let tmp_instr:Instructions = {Instructions{
            direction: one_direction, spaces: num_spaces
        } };
        vec_instructions.push(tmp_instr);
    }

    // now move around
    // start the tail hashmap with the starting position
    let new_tail_coords = tail_loc.clone();
    let mut new_tail_coords2 = tail_loc.clone();
    add_listing ( &mut hash_tail,  new_tail_coords, 1 );
//    new_tail_coords2.x=1;
//    hash_tail.entry(new_tail_coords2).and_modify(|k| *k += 1).or_insert(1);
    let hashkey1 = String::from("0-0");
    let hashkey2 = String::from("1-0");
    let hashkey3 = hashkey1.clone();
    hash_tail2.entry(hashkey1).and_modify(|k| *k += 1).or_insert(1);
//    hash_tail2.entry(hashkey2).and_modify(|k| *k += 1).or_insert(1);
//    hash_tail2.entry(hashkey3).and_modify(|k| *k += 1).or_insert(1);
    
    
    for curr_instr in vec_instructions{
        (head_loc,tail_loc) = move_head(curr_instr,head_loc,tail_loc, &mut hash_tail2);

        println!("hold up");
    }




}

fn move_head (local_inst:Instructions,local_head_loc:Coordinates,local_tail_loc:Coordinates,local_hashmap_tail:&mut  HashMap<String,i32> ) -> (Coordinates,Coordinates){
//    let mut updated_head_loc:Coordinates = local_head_loc;
    let x_pos= local_head_loc.x.clone();
    let y_pos = local_head_loc.y.clone();
    let mut updated_head_loc:Coordinates = Coordinates { x: x_pos, y: y_pos };
    let spaces_to_move = local_inst.spaces.clone();
    let mut tail_x_pos = local_tail_loc.x.clone();
    let mut tail_y_pos = local_tail_loc.y.clone();
    let mut updated_tail_loc:Coordinates = Coordinates { x: tail_x_pos, y: tail_y_pos };
/*
the move the head all at once creates a problem with moving the tail to keep up
need to figure out for each one how to handle that 
choice - some way to do a for loop for each one:
challenge - that only works if tail is in line with head:
same y for a L or R
same x for a U or D

if it's not in the same line, then ??? maybe it doesn't matter - the Tail goes to 
the "current -1 " for head (snapping it to the same x or y?)
-- HAVE TO MOVE ONE AT A TIME

*/
    println!("local_inst:   local head:  ");

    match local_inst.direction {
        'R' => {
            for mover in 1..spaces_to_move+1 {
                updated_head_loc.x = x_pos + mover;   
                let (near_head, x_to_move, y_to_move) = tail_near_head(&updated_head_loc, &updated_tail_loc);
                if !near_head {
                    println!("need to move");
                    updated_tail_loc.x = updated_tail_loc.x + x_to_move;
                    updated_tail_loc.y = updated_tail_loc.y + y_to_move;
                    let our_tail_copy = updated_tail_loc.clone();
                    let our_tail_copy2 = updated_tail_loc.clone();
                    let our_tail_copy3 = updated_tail_loc.clone();
                    
                //    add_listing (  local_hashmap_tail,  our_tail_copy, 1 );
                //    local_hashmap_tail.entry(our_tail_copy).and_modify(|k| *k += 1);
                    let hashkey2 = String::from("1-0");
                    let hashkey3 = String::from("1-0");
                    local_hashmap_tail.entry(hashkey2).and_modify(|k| *k += 1).or_insert(1);
                    local_hashmap_tail.insert(hashkey3,1);
                    println!("what happpened?");
                    //    local_hashmap_tail.entry(hashkey2)
                //    .and_modify(|e| { *e += 1 })
                //    .or_insert(1);

                /*     if local_hashmap_tail.contains_key(&our_tail_copy3) {
                        println!("key val {:?}", local_hashmap_tail.get(&our_tail_copy3));
                    } else {
                        local_hashmap_tail.insert(our_tail_copy3, 1);
                    }
                */
                    println!("did we add anything?");

                }
    
            }
//            updated_head_loc.x = x_pos + spaces_to_move;
//            let (near_head,x_to_move,y_to_move) = tail_near_head(&updated_head_loc, &updated_tail_loc);

        }
        'L' => {
            updated_head_loc.x = x_pos - spaces_to_move;

        }
        'U' => {
            updated_head_loc.y = y_pos + spaces_to_move;

        }
        'D' => {
            updated_head_loc.y = y_pos - spaces_to_move;

        }
        _ => println!("** THIS SHOULDN'T HAPPEN"),
    }
    return (updated_head_loc, updated_tail_loc);
}


/*===  tail_near_head =======
Is the tail "next to the head"
*/
fn tail_near_head (local_head_loc:&Coordinates,local_tail_loc:&Coordinates) -> (bool,i32,i32){
let mut close:bool = true;
let mut x_move:i32 =0;
let mut y_move:i32 =0;

let x_delta:i32 = (local_head_loc.x - local_tail_loc.x).try_into().unwrap();
let abs_x_delta = x_delta.abs();
let y_delta:i32 = (local_head_loc.y - local_tail_loc.y).try_into().unwrap();
let abs_y_delta = y_delta.abs();
if x_delta > 1 { // positive X
    close = false;
    x_move= 1;
}  
if x_delta < -1 { //negative x
    close = false;
    x_move= -1;    
}
if y_delta > 1 { // positive Y
    close = false;
    y_move= 1;
}  
if y_delta < -1 { // negative Y
    close = false;
    y_move= -11;
}  
    // how many to move? 
    /* OLD WAY - TO DELETE 

if (abs_x_delta > 1 && abs_y_delta ==0) { //only x move - same row
    close = false;
    
 ways we move:
same row (y=y): + / - X
same column (x=x): + / - Y
Dif row / dif coll
y !=y , x !=x : 
one closer x and once closer y

If the head is ever two steps directly up, down, left, or right from the tail,
the tail must also move one step in that direction so it remains close enough:
    */


return (close,x_move,y_move); // if we fall through the the end then our "far away" if's weren't hit
}

//fn add_listing<'a> (file_listing: &mut HashMap<'a> String, u32>, key_str:<'a> String>, val_u32:u32 ){
//   HashMap<String, u32>
fn add_listing<'a> (tail_listing: &mut HashMap < Coordinates, i32>, key_coord: Coordinates, val_i32:i32 ){
    tail_listing.entry(key_coord)
    .and_modify(|e| { *e += val_i32 })
    .or_insert(val_i32);
 println!("added a thing");
 }

/*
hashmap add or update
let mut my_map = HashMap::new();
my_map.insert("a", 1);
my_map.entry("a").and_modify(|k| *k += 10);
assert_eq!(my_map[&"a"], 11);

*/