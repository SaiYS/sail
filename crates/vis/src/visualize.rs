pub trait Visualize {
    fn visualize(&self) -> String;
}

macro_rules! impl_visualize {
    ($($t:ty),+) => {
        $(
            impl Visualize for $t {
                fn visualize(&self) -> String {
                    format!("{}", self)
                }
            }
        )+
    };
}

impl_visualize! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    String, &str, char
}

impl<T: Visualize, U: Visualize> Visualize for (T, U) {
    fn visualize(&self) -> String {
        format!("{} {}", self.0.visualize(), self.1.visualize())
    }
}

impl<T: Visualize, U: Visualize, V: Visualize> Visualize for (T, U, V) {
    fn visualize(&self) -> String {
        format!(
            "{} {} {}",
            self.0.visualize(),
            self.1.visualize(),
            self.2.visualize()
        )
    }
}

impl<T: Visualize, U: Visualize, V: Visualize, W: Visualize> Visualize for (T, U, V, W) {
    fn visualize(&self) -> String {
        format!(
            "{} {} {} {}",
            self.0.visualize(),
            self.1.visualize(),
            self.2.visualize(),
            self.3.visualize()
        )
    }
}

impl<T: Visualize, U: Visualize, V: Visualize, W: Visualize, X: Visualize> Visualize
    for (T, U, V, W, X)
{
    fn visualize(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.0.visualize(),
            self.1.visualize(),
            self.2.visualize(),
            self.3.visualize(),
            self.4.visualize()
        )
    }
}

#[macro_export]
macro_rules! vis {
    () => {
        println!();
    };

    ($x:expr) => {
        println!("{}", $x.visualize());
    };

    ($xs:expr => $sep:literal) => {
        vis!(itertools::Itertools::join(
            &mut $xs.iter().map(|e| e.visualize()),
            $sep
        ));
    };

    ($xs:expr => $sep1:literal => $sep2:literal) => {
        vis!(itertools::Itertools::join(
            &mut $xs
                .iter()
                .map(|ys| itertools::Itertools::join(&mut ys.iter().map(|e| e.visualize()), $sep1)),
            $sep2
        ));
    };

    ($xs:expr => $sep1:literal => $sep2:literal => $sep3:literal) => {
        vis!(itertools::Itertools::join(
            &mut $xs.iter().map(|ys| itertools::Itertools::join(
                &mut ys.iter().map(|zs| itertools::Itertools::join(
                    &mut zs.iter().map(|e| e.visualize()),
                    $sep1
                )),
                $sep2
            )),
            $sep3
        ));
    };
}
