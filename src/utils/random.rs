

pub struct Random {
    seed: u32,
}

impl Random {
    pub fn init(s: u32) -> Self {
        let mut seed = s & 0x7fff_ffff;
        if seed == 0 || seed == 2147483647 {
            seed = 1;
        }
        Self { seed }
    }

    pub fn next(&mut self) -> u32 {
        let m: u32 = 2147483647;
        let a: u64 = 16807;
        let product = self.seed as u64 * a;
        self.seed = ((product >> 31) + (product & m as u64)) as u32;
        if self.seed > m {
            self.seed -= m;
        }

        self.seed
    }

    pub fn uniform(&mut self, n: u32) -> u32 {
        self.next() % n
    }

    pub fn one_in(&mut self, n: u32) -> bool {
        self.next() % n == 0
    }

    pub fn skewed(&mut self, max_log: u32) -> u32 {
        let log = self.uniform(max_log + 1);
        self.uniform(1 << log)
    }
}