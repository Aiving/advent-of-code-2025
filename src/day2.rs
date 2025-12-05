pub const INPUT: &str = include_str!("../assets/day2-input.txt");

fn is_invalid(id_str: &str, simple: bool) -> bool {
    if simple {
        let half = id_str.len() / 2;

        id_str[0..half] == id_str[half..]
    } else {
        let size = id_str.len();
        let mut invalid = false;

        for i in 2..=size {
            if !size.is_multiple_of(i) {
                continue;
            }

            let half = size / i;

            for j in 0..i {
                if id_str[(j * half)..((j + 1) * half)] == id_str[0..half] {
                    invalid = true;
                } else {
                    invalid = false;

                    break;
                }
            }

            if invalid {
                break;
            }
        }

        invalid
    }
}

pub fn solution() {
    let mut first_sum = 0;
    let mut second_sum = 0;

    for value in INPUT.split(',') {
        let mut values = value.split('-');
        let Some(from) = values.next() else { break };
        let Some(to) = values.next() else { break };

        let from_invalid = from.len() >= 2 && is_invalid(from, true);
        let to_invalid = to.len() >= 2 && is_invalid(to, true);

        let from = from.parse::<i64>().unwrap();
        let to = to.parse::<i64>().unwrap();

        if from_invalid {
            first_sum += from;
            second_sum += from;
        }

        if to_invalid {
            first_sum += to;
            second_sum += to;
        }

        if !from_invalid && !to_invalid {
            let min = from.min(to);
            let max = from.max(to);

            for i in 0..(max - min) {
                let value = min + i;
                let value_str = value.to_string();

                if is_invalid(&value_str, true) {
                    first_sum += value;
                    second_sum += value;
                } else if is_invalid(&value_str, false) {
                    second_sum += value;
                }
            }
        }
    }

    println!("Part 1 answer: {first_sum}");
    println!("Part 2 answer: {second_sum}");
}
