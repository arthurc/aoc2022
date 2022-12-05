pub fn calculate_calories_per_elf(input: &str) -> Vec<u32> {
    input.lines().map(parse_line).fold((None, Vec::new()), |(previous_n, mut result), n| {
        match (previous_n, n) {
            (None, Some(n)) => result.push(n),
            (Some(_), Some(n)) => if let Some(result) = result.last_mut() {
                *result += n;
            },
            _ => ()
        };

        (n, result)
    }).1
}

fn parse_line(line: &str) -> Option<u32> {
    line.trim().parse().ok()
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(Some(1000), parse_line("1000"));
        assert_eq!(None, parse_line(""))
    }

    #[test]
    fn test_example() {
        let calories_per_elf = calculate_calories_per_elf("
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000");

            assert_eq!(vec![6000, 4000, 11000, 24000, 10000], calories_per_elf);
    }
}
