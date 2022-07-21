# Cellular Automata

These CA were popularized by Wolfram. We have a string of bits and a set of rules for creating the next generation. These rules decide on the next bit based on the current value and that of the two direct neighbours. Since there are three bits involved in this decision, there are only eight input patterns to consider. Each input pattern yields either 1 or 0, so we have 256 possible cellular automata.

```rust file=src/lib.rs
pub trait Universe {
    fn single_bit_on(size: Option<usize>) -> Self;
    fn next(&self, rule: u8) -> Self;
    fn print(&self);
}

pub struct FiniteUniverse {
    bits: Vec<u8>
}

fn apply_rule(rule: u8, bits: (u8, u8, u8)) -> u8 {
    let index = bits.0 * 4 + bits.1 * 2 + bits.2;
    (rule >> index) & 1
}

impl Universe for FiniteUniverse {
    fn single_bit_on(size_: Option<usize>) -> FiniteUniverse {
        let size = size_.unwrap_or(256);
        let mut bits = vec![0;size];
        bits[size/2] = 1;
        FiniteUniverse { bits }
    }

    fn next(&self, rule: u8) -> FiniteUniverse {
        let size = self.bits.len();
        let mut next_bits = vec![0;size];
        for (i,w) in self.bits.windows(3).enumerate() {
            next_bits[i+1] = apply_rule(rule, (w[0], w[1], w[2]));
        }
        FiniteUniverse { bits: next_bits }
    }

    fn print(&self) {
        let s:String = self.bits.iter().map(
            |&b| if b == 0u8 { ' ' } else { '#' }).collect();
        println!("{}", s);
    }
}

```

```rust file=src/main.rs
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
```

