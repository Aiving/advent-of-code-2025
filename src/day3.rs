pub const INPUT: &str = include_str!("../assets/day3-input.txt");

fn max_joltage(digits_per_bank: u32) -> u64 {
    let mut max_joltage = 0;

    for line in INPUT.lines() {
        let size = line.len();
        let mut result = 0;
        let mut last_index = None;

        for i in 0..digits_per_bank {
            let mut max = None;

            for (i, digit) in line
                .chars()
                .enumerate()
                .take(size - (digits_per_bank - 1 - i) as usize)
                .skip(last_index.map_or(0, |index| index + 1))
                .filter_map(|(i, digit)| digit.to_digit(10).map(|digit| (i, digit)))
            {
                if max.is_none_or(|(_, max)| digit > max) {
                    max = Some((i, digit));
                }
            }

            if let Some((index, max)) = max {
                result += 10u64.pow(digits_per_bank - 1 - i) * u64::from(max);
                last_index = Some(index);
            }
        }

        max_joltage += result;
    }

    max_joltage
}

pub fn solution() {
    println!("Part 1 answer: {}", max_joltage(2));
    println!("Part 2 answer: {}", max_joltage(12));
}
