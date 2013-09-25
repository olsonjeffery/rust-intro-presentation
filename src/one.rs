// one - safe

pub struct Knapsack {
    invisible: bool
    // play along and let's just assume
    // our knapsack has some kind of underlying state
    // that requires it to have a dtor 
}
// the "dtor trait"

pub fn query_knapsack_status(idx: uint) -> Knapsack {
    // this is our underlying "knapsack datastore"
    let knapsacks = [Knapsack { invisible: false }, .. 8];
    return knapsacks[idx];
}

fn main() {
    let ks = query_knapsack_status(0);
    println!("bleh");
}
