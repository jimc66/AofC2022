/*
main for advent of code
simple structure is a module per day (day##.rs)
that is updated each day 
- probably a lot of refactoring day over day, but this preserves each days
challenges 

this is how one rust newbie handles this   :)

*/

mod day2;

// pub mod test2;

fn main() {
    day2::day2();
/*     let mut max_num: i64 = 0;
    let mut max2_num: i64 = 0;
    let mut max3_num: i64 = 0;
    let mut curr_num: i64 = 0;
    
    let file_contents = fs::read_to_string(FNAME)
    .expect("LogRocket: Should have been able to read the file");
//println!("info.txt context =\n{file_contents}");
for line in file_contents.lines() {
    println!("{}", line);
    let i:i64= match line.parse::<i64>() {
        Ok(i) => i,
        Err(_e) => -1,
      

      };
      //println!("as a num {}",i );
      if i > 0 {
        curr_num = curr_num + i;}

        else
        {
            if curr_num > max_num { // replace top value and bump existing down 1 each
                max3_num = max2_num;
                max2_num = max_num;
                max_num = curr_num;
                curr_num = 0;
            }
            else if curr_num > max2_num{
                max3_num = max2_num;
                max2_num = curr_num;
                curr_num = 0;
            }   else if  curr_num > max2_num {
                max3_num = curr_num;
                curr_num = 0;
            }
            
            else {
                curr_num = 0;
            }
        }
      }
println!("max number {}", max_num);
let tot = max2_num+max3_num+max_num;
println!("top 3 tot {}", tot);
*/

//day1();

}


//
//

/* 
    let mut file = File::open('day1_1.txt')
    .expect("File not found");
    let mut data = String::new();
   file.read_to_string(&mut data)
    .expect("Error while reading file");
   println!("{}", data);
*/

/*
    read_file_vec(FNAME);

   another_function();
*/
  //  println!("Hello, world!");

    
    



/* 
fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    /*    let data = fs::read(filepath)?;
    Ok(data)
    */
}
*/