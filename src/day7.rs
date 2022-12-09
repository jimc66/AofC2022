use std::fs;
use std::collections::HashMap;

//use substring::Substring;

 /* DAY 7 

   THINKING
   data structures we need
   list of file paths (as a vector - char with paths 
push /
push /fdssdf

then track - current dir
go with hashmap
key is a directory or a file
trailing / means its a dir
the size is the value
if you add a value to a key that's there it increaes the value

HASH MAP
then we could have a key value (hashmap?)
key /ful/dir/path/filename

value file size
or do we want some type of structure
directory

then how to know the full size of dirs?
/fddsf/dfsdfs/ so a key with a trailing / is a dir
and the value is size



)
   strait up 
   file path

   Key  /dir path 
   Value 


   /
   /fdjkdfs
   /kfd;dfs/fddfskl

   or

   
   */


pub fn day7 () {
 /*  
    Pretty much for all days - read in the file
    */    
    //const FNAME : &str = "./files/day5.txt" ;
    const FNAME : &str = "./files/test7.txt" ;

    let file_contents = fs::read_to_string(FNAME)
    .expect("LogERROR: Should have been able to read the file");

    let mut current_dir:String = "/".to_string(); // keep track of where you are 
    let mut file_list2:HashMap<String, u32> = HashMap::new(); 

//   let scratch:String = current_dir.to_owned()+&dir_to_add;
//   let scratch2 = scratch.clone();
//   let  key1:&str = &scratch2.to_owned();
   let val1:u32 =0;

   file_list2.entry(current_dir.to_string())
   .and_modify(|e| { *e += val1 })
   .or_insert(val1);

    for line in file_contents.lines() {
      //  first char: $(commands), char (for any file entry or directory),0-9 (for a file)   
      let first_char = line.chars().nth(0).unwrap();
      match first_char {
         '$' => { // command logic $cd $ ls
            println!("Command flow {}",line);
            if line == "$ ls" {
               // if it's an ls, just eat the line
               println!("ls command {}", line);
            }
            else if &line[..6] == "$ cd ." { // take a risk since . help with out of bounds
               println!("cd .. command: {}", line);
               //currdir - remove last char last / then rfind /, then chop to the /
               let orig_dir_sz = current_dir.len(); 
               let current_dir_notrail:&str = &current_dir[0..orig_dir_sz-1]; // chop last char            
               let pos = current_dir_notrail.rfind('/').unwrap(); // new last /
               println!("last / is at {:?}", pos);
               let tmper:String = current_dir_notrail.to_string();
               let temp_new_current = &tmper[0..pos]; 
               current_dir = temp_new_current.to_string()+"/"; // chop it down
              /*  not for this???
               let dir_to_add:&str = &line[6..];
               let scratch = current_dir+&dir_to_add;
               current_dir = scratch.to_owned();
               */

               println!("new current dir: {}", current_dir);
               // move up a directory / trim current_dir
            } 
            else { // we are cd'ing down
               let vec_str: Vec<&str> = line.split_whitespace().collect();
               println!("part 0:{} part 1:{} part 2:{}", vec_str[0], vec_str[1], vec_str[2]);
               let dir_to_add:&str = &vec_str[2].to_owned();
               current_dir = current_dir+&dir_to_add+"/";
               println!("dir to add: {} \nresult: {}", dir_to_add,current_dir);
               let scratch:String = current_dir.to_owned()+&dir_to_add;
               let val1:u32 =0;

               /* THE WORKING ONE
               file_list2.entry(current_dir.to_string())
               .and_modify(|e| { *e += val1 })
               .or_insert(val1);
               */

               add_listing (&mut file_list2,  current_dir.to_string(), val1 );

            }

         } 
         '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => { // file logic
            println!("file flow {}",line);
            let vec_str: Vec<&str> = line.split_whitespace().collect();
            println!("part 0:{} part 1:{}", vec_str[0], vec_str[1]);
            let file_size:u32 = vec_str[0].to_string().parse::<u32>().unwrap();
            let just_file:String = vec_str[1].to_string();
            let full_file:String = current_dir.clone() + &just_file;
            println!("adding file :{} \n size:{}", full_file, file_size);
            add_listing (&mut file_list2,  full_file, file_size );
            
            // now update the size of the current dir
            println!("updating directory size for: {}", current_dir);
            add_listing (&mut file_list2,  current_dir.to_string(), file_size );
            println!("done with file update");
            // we also need to walk UP the dir tree and add the new file size to all parent directories
            //currdir - remove last char last / then rfind /, then chop to the /
            let mut cur_last_slash_pos = current_dir.len();
            let mut working_directory = current_dir.clone();
            while cur_last_slash_pos > 1 {
               let working_dir_sz = working_directory.len(); 
               let working_dir_notrail:&str = &&working_directory[0..working_dir_sz-1]; // chop last char            
               let new_last_pos = working_dir_notrail.rfind('/').unwrap(); // new last / - not including current
               cur_last_slash_pos = new_last_pos;
               println!("last / is at {:?}", new_last_pos);
               let tmper:String = working_dir_notrail.to_string();
               let temp_new_working = &tmper[0..new_last_pos]; 
               working_directory = temp_new_working.to_string()+"/"; // chop it down
               add_listing (&mut file_list2,  working_directory.to_string(), file_size );
            }


         }
         _ => { // dir line logic if not a $ or num
            println!("dir flow {}",line);

         }
      }

    }



    //let mut dir_list:Vec<String> = Vec::new();
    //let mut file_list:HashMap<String, u32> = HashMap::new();
      
   



    //  add_listing (file_listing: &mut HashMap<&str, u32>, key_str:&str,val_u32:u32 )    
    



println!("let's see if this works so far");

// now we have all files and the dir lists the size of for all dirs and sub-dirs)
// collect up the dirs: 
/* iteratate (whole hashmap)
    for the items that end in / 
    that have a size <= 100000
    add to total 
*/
   let mut total_size = 0; 
   let max_size:u32 = 100000;
   for (key, value) in &file_list2 {
      println!("{} / {}", key, value);
      let item_len = key.len();
      let last_char = key.chars().nth(item_len-1).unwrap();
      if last_char == '/' {
         if value <= &max_size {
            total_size = total_size+value;
         }
      }

   //   map.remove(key);
   }
   println!("size answer: {}", total_size);
 println!("done done")  

}

//fn add_listing<'a> (file_listing: &mut HashMap<'a> String, u32>, key_str:<'a> String>, val_u32:u32 ){
//   HashMap<String, u32>
fn add_listing<'a> (file_listing: &mut HashMap < String, u32>, key_str: String, val_u32:u32 ){
   file_listing.entry(key_str)
   .and_modify(|e| { *e += val_u32 })
   .or_insert(val_u32);
println!("added a thing");
}

/*
OLD TESTING STUFF

    dir_list.push(String::from("/"));
    dir_list.push(String::from("/foobar/"));
    file_list.insert(String::from("/"), 0);
    file_list.insert(String::from("/foobar/file"), 22);
    file_list.insert(String::from("/foobar/file2"), 66);
    let mut file_list2:HashMap<&str, u32> = HashMap::new();   
    let key1:&str = "key1";
    let val1:u32 =22;
    let key2:&str = "key2";
    let val2:u32 =40;
    let key3:&str = "key3";
    let val3:u32 =200;
    file_list2.insert(key1,val1 );
    let mut currsize:u32 =100;
    file_list2.entry(key1)
      .and_modify(|e| { *e += currsize })
      .or_insert(currsize);

      file_list2.entry(key2)
      .and_modify(|e| { *e += currsize })
      .or_insert(val2);


    file_list.entry(foobarkey)
      .and_modify(|e| { *e += 22 })
      .or_insert(20);

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland",44);

    map.entry("poneyland")
       .and_modify(|e| { *e += currsize })
       .or_insert(20);
    //assert_eq!(map["poneyland"], 42);

    map.entry("poneyland2")
    .and_modify(|e| { *e += 2 })
    .or_insert(25);

*/