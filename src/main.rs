use safe::knapsack;
mod safe;
mod concurrent;
mod practical;

fn main() {
    println!("~~~~~~~~~~");
    practical::hello::world(); //00
    println!("is knapsack 0 invisible? {}", knapsack::query_knapsack_status(0)); //01
}
