use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}
