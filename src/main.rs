mod tower_of_hanoi;

fn main() {
    let mut x = tower_of_hanoi::puzzle_with_discs(3);

    println!("{}", x);

    x.solve(|p| println!("{}", p));
}
