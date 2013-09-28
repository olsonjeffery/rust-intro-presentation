// one - safe

pub struct Knapsack {
    invisible: bool
    // play along and let's just assume
    // our knapsack has some kind of underlying state
    // that requires it to have a dtor 
}
// the "dtor trait"
impl Drop for Knapsack {
    fn drop(&mut self) {
        // ...
    }
}

impl Clone for Knapsack {
    fn clone(&self) -> Knapsack {
        Knapsack { invisible: self.invisible }
    }
}

pub fn query_knapsack_status(idx: uint) -> bool {
    // this is our underlying "knapsack datastore"
    let mut knapsacks: ~[Knapsack] = ~[];
    knapsacks.push(Knapsack{ invisible: true });
    return knapsacks[idx];
}

