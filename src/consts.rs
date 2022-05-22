pub const INF: i64 = 2000000000;
pub const FNI: i64 = -2000000000;
pub const ALPHABET_LARGE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABET_SMALL: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ADJ4: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const ADJ8: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
