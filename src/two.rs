// 02 - practical

enum Alignment {
    Good,
    Neutral,
    Evil
}

pub fn rust_is_explicit() {
    // fixed sized, stack-allocated
    let mut first_16_players: [(uint, Alignment)] = [(0, Neutral), .. 16];
    first_16_players[0] = (1, Good);
    first_16_players[0] = (2, Evil);
    // dynamically sized, heap-allocated
    let mut everyone_else: ~[(uint, Alignment)] = ~[];
    // printing the id/alignment of all player-slots that aren't stubbed
    for &p in first_16_players {
        match p {
            (0, Neutral) => {},
            (id, align) => { println!("id {}, align: {}",
                                      id, align); }
            // "catch-all" arm; underscore is shorthand for
            // "I don't care about this value"
            // _ => {}
        }
    }
}