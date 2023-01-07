use proconio::input;
use proconio::marker::{Bytes};

fn main() {
    input! {
        s: Bytes,
    }

    let mut cnt = 0;

    for v in s {
        if v == b'1' { cnt += 1; }
    }

    println!("{}", cnt);
}
