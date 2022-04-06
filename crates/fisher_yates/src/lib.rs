use rand::{thread_rng, Rng};

pub fn shuffle<T>(mut v: Vec<T>) -> Vec<T> {
    let n = v.len();
    let mut rng = thread_rng();
    for i in (1..n).rev() {
        let j = rng.gen_range(0, i + 1);
        v.swap(i, j);
    }
    v
}

pub trait Shufflable {
    fn shuffle(self) -> Self;
}

impl<T> Shufflable for Vec<T> {
    fn shuffle(self) -> Self {
        shuffle(self)
    }
}
