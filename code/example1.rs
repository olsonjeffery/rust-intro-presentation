// example1.rs
use std;
use std::task::spawn_sched;
use std::comm::oneshot;
use std::option::{Option, Some, None};

struct HotDog {
    sausage: Sausage,
    toppings: Toppings,
    bun: Bun
}
enum Sausage {
    Bratwurst,
    Tofurkey,
    EverythingElse(~str) // i mean, really
}
enum Toppings {
    Sauerkraut,
    Ketchup(bool) // is it Fair Trade?
}
enum Bun {
    SomethingDecadent,
    EwwwCarbs // high in fiber
}
impl HotDog {
    fn new(s: Sausage) -> Option<HotDog> {
        match Sausage {
            Bratwurst => HotDog {
                sausage: BratWurst,
                toppings: Sauerkraut,
                bun: SomethingDecadent
            },
            Tofurkey => HotDog {
                sausage: Tofurkey,
                toppings: Ketchup(true),
                bun: EwwwCarbs
            },
            _ => None // no hotdog for you!
        }
    }
}
fn main() {
    let (port, chan) = oneshot(); 
    do spawn_sched {
        let brat = HotDog(Bratwurst);
        chan.send(brat);
    }
    // parallel hotdog assembly!
    let ewww_yuck = HotDog(Tofurkey);
    let brat = port.recv();
}