// ~\~ language=Rust filename=src/lib.rs
// ~\~ begin <<lit/ca.md|src/lib.rs>>[0]
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

// ~\~ end
