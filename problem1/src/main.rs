/// `solution` function solves the first Project Euler problem

pub fn solution(max: u64) -> u64 {
    let mut result = 0;
    for i in 0..max {
      if i % 3 == 0 || i % 5 == 0 {
        result += i;
      }
    }
    return result;
}
fn main() {
    println!("{}", solution(5));
}