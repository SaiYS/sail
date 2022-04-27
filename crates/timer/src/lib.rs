use std::time::Instant;

pub type MilliS = u128;

const DEFAULT_TL: u128 = 2000;
const DEFAULT_REDUNDANCY: u128 = 50;

pub struct Timer {
    initilized: Instant,
    tl: MilliS,
    redundancy: MilliS,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            initilized: Instant::now(),
            tl: DEFAULT_TL,
            redundancy: DEFAULT_REDUNDANCY,
        }
    }
}

impl Timer {
    pub fn new(tl: MilliS, redundancy: MilliS) -> Self {
        Self {
            initilized: Instant::now(),
            tl,
            redundancy,
        }
    }

    pub fn duration(&self) -> MilliS {
        (Instant::now() - self.initilized).as_millis()
    }

    pub fn check(&self) -> bool {
        self.duration() > self.tl - self.redundancy
    }
}
