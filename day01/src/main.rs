fn main() {
    let mut calculated_calculated = day01::calculate_calories_per_elf(include_str!("../input.txt"));
    calculated_calculated.sort();
    calculated_calculated.reverse();

    for (index, calories) in calculated_calculated.iter().enumerate() {
        println!("{:>3}. {}", index + 1, calories);
    }
}
