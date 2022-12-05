fn main() {
    let mut calculated_calculated = day01::calculate_calories_per_elf(include_str!("../input.txt"));
    calculated_calculated.sort();
    calculated_calculated.reverse();

    println!("part1: {}", calculated_calculated.first().unwrap_or(&0));
    println!("part2: {}", calculated_calculated.iter().take(3).sum::<u32>());
}
