use std::{fs, num};
use std::str::Split;    
use std::collections::HashMap;

pub fn day4 () {
   const FNAME : &str = "./files/day4.txt" ;
// const FNAME : &str = "./files/test4.txt" ;


/* 
Pretty much for all days - read in the file
 */    
    let file_contents = fs::read_to_string(FNAME)
    .expect("LogERROR: Should have been able to read the file");
/*
DAY 4 PART A / 1
*/
// Go through all the lines and create the integer pairs

let mut number_matches:i64=0;
let mut partial_matches:i64=0; //for part 2

for line in file_contents.lines(){
    let mut elf1: HashMap< i64,bool> = HashMap::new();
    let mut elf2: HashMap<i64, bool> = HashMap::new();
    let line_parts = line.split(",");
    let mut current_elf:usize=1;
    for elf_group in line_parts {
        println!("range {}", elf_group); 
        let number_parts_as_str: Split<&str> = elf_group.split("-");
        let mut start_num:i64 =-1;
        let mut end_num:i64 =-1;
        
        for actual_sections in number_parts_as_str {
            let mut insert_section_num=-1;
            println!("section: {}", actual_sections);    // go through the 2 values
                let section_num = actual_sections.parse::<i64>().unwrap();
                if start_num == -1 || start_num == section_num { //it's our first one, or it matches first one
                    insert_section_num = section_num;
                    start_num = section_num;
                    if current_elf == 1 { 
                        elf1.insert( insert_section_num,true);  // insert works even if replacing
                    }
                    else {
                        elf2.insert( insert_section_num,true);  // insert works even if replacing
                    }
                }
                else {
                        for iterator_val in  start_num..section_num+1 {
                        if current_elf == 1 { 
                            elf1.insert( iterator_val,true);  // insert works even if replacing
                        }
                        else {
                            elf2.insert( iterator_val,true);  // insert works even if replacing
                        }
                    }   
                }  
          

        }
        current_elf=current_elf+1; // we're assuming there are 2 groups in this thing
    }
// now that they are stored - check the 2 assignments
let elf1_size = elf1.keys().len();
let elf2_size = elf2.keys().len();
// a little brutish - see if the shorter one is in the bigger one
if elf2_size >=elf1_size { // elf2 is bigger or equal - iterated through elf 1
    let mut full_match:bool = false;
    let mut keep_matching:bool = true;

'iter_loop1:    for  (k, v) in elf1.iter_mut() {
        if elf2.contains_key(k){
            keep_matching=true;
        }
        else {
            keep_matching=false;
            break 'iter_loop1;
        }
      }
      if keep_matching == true {
        number_matches = number_matches+1;

        } 
       
    }
else { // elf1 is bigger
    let mut full_match:bool = false;
    let mut keep_matching:bool = true;

'iter_loop2:    for  (k, v) in elf2.iter_mut() {
        if elf1.contains_key(k){
            keep_matching=true;
        }
        else {
            keep_matching=false;
            break 'iter_loop2;
        }
      }
      if keep_matching == true {
        number_matches = number_matches+1;

        } 
        
    
}
// PUT PART 2 COMPARISION HERE
// =========
// a little brutish - see if the shorter one is in the bigger one
if elf2_size >=elf1_size { // elf2 is bigger or equal - iterated through elf 1
    let mut full_match:bool = false;
    let mut partial_match:bool = false;
    let mut keep_matching:bool = true;

'iter_loop3:    for  (k, v) in elf1.iter_mut() {
        if elf2.contains_key(k){
            keep_matching=true;
            partial_match=true;
            break 'iter_loop3; // as soon as any match break out and count it
        }
        else {
            keep_matching=true; // sanity check - no endless loop as the for will kill it
   
        }
      }
        if partial_match ==true{
            partial_matches = partial_matches +1;
        }
    }
else { // elf1 is bigger
    let mut full_match:bool = false;
    let mut partial_match:bool = false;
    let mut keep_matching:bool = true;

'iter_loop4:    for  (k, v) in elf2.iter_mut() {
        if elf1.contains_key(k){
            keep_matching=true;
            partial_match=true;
            break 'iter_loop4;
            
        }
        else {
            keep_matching=false;
            
        }
      }
     
        if partial_match ==true{
            partial_matches = partial_matches +1;
        }
    
}



}
println!("number matches {}", number_matches);
println!("number partial matches {}", partial_matches);

}