mod tower_of_hanoi;

fn main() {
    let mut puzzle = tower_of_hanoi::puzzle_with_discs(3);

    println!("{}", puzzle);

    puzzle.solve(|p| println!("{}", p));
}
