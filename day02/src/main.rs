fn main() {
    println!(
        "part1: {}",
        day02::calculate_total_score(include_str!("../input.txt"))
    );
    println!(
        "part2: {}",
        day02::calculate_predicted_score(include_str!("../input.txt"))
    );
}
