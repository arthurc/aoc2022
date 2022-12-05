fn main() {
    println!(
        "part1: {}",
        day03::calculate_combined_priority_for_shared_items(include_str!("../input.txt"))
    );
    println!(
        "part2: {}",
        day03::calculate_badge_priority_for_groups::<3>(include_str!("../input.txt"))
    );
}
