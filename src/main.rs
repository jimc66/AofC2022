/*
main for advent of code
simple structure is a module per day (day##.rs)
that is updated each day 
- probably a lot of refactoring day over day, but this preserves each days
challenges 

this is how one rust newbie handles this   :)

*/

mod day5; //filename by day

// pub mod test2;

fn main() {
    day5::day5();
}