// ~\~ language=Rust filename=src/main.rs
// ~\~ begin <<lit/ca.md|src/main.rs>>[0]
use egui_automata::*;

fn main () {
    let rule = 30;
    let mut universe = FiniteUniverse::single_bit_on(Some(256));
    universe.print();
    for _ in 0..128 {
        universe = universe.next(rule);
        universe.print()
    }
}
// ~\~ end
