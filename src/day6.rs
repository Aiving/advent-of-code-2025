pub const INPUT: &str = include_str!("../assets/day6-input.txt");

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

pub fn solution() {
    let lines = INPUT.lines().count();
    let Some(mut operators) = INPUT.lines().last().and_then(|row| {
        row.split_whitespace()
            .map(|value| match value.chars().next() {
                Some('+') => Some((Operator::Add, Some(0))),
                Some('*') => Some((Operator::Mul, None)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
    }) else {
        panic!("invalid operators row")
    };

    for row in INPUT.lines().take(lines - 1) {
        for (column, number) in row
            .split_whitespace()
            .filter_map(|column| column.parse::<usize>().ok())
            .enumerate()
        {
            operators[column].1 = operators[column].1.map_or(Some(number), |value| {
                Some(match &operators[column].0 {
                    Operator::Add => value + number,
                    Operator::Mul => value * number,
                })
            });
        }
    }

    println!(
        "Part 1 answer: {}",
        operators
            .iter()
            .map(|(_, value)| value.unwrap_or_default())
            .sum::<usize>()
    );

    let lines = INPUT.lines().count();
    let mut operators = Vec::new();

    if let Some(row) = INPUT.lines().last() {
        let mut row = row.split(' ').map(|c| c.chars().next()).peekable();

        while let Some(maybe_operator) = row.next() {
            match (maybe_operator, row.peek()) {
                // [* ]
                (Some('*' | '+'), Some(None)) => operators.push((
                    if maybe_operator == Some('*') {
                        Operator::Mul
                    } else {
                        Operator::Add
                    },
                    1usize,
                    Vec::with_capacity(lines - 1),
                )),
                // [  ]
                (None, Some(Some('*' | '+') | None) | None) => {
                    if let Some((_, width, _)) = operators.last_mut() {
                        *width += 1;
                    }
                }
                arm => println!("uncatched arm: {arm:?}"),
            }
        }
    }

    for mut row in INPUT.lines().take(lines - 1) {
        for (_, width, numbers) in &mut operators {
            let (column, new_row) = row.split_at(*width);

            numbers.push(column);

            if !new_row.is_empty() {
                row = &new_row[1..];
            }
        }
    }

    let mut sum = 0;

    for (operator, width, numbers) in &operators {
        let mut operator_result = match operator {
            Operator::Add => 0,
            Operator::Mul => 1,
        };

        for i in 0..*width {
            let mut result = String::with_capacity(numbers.len());

            for number in numbers {
                if let Some(num) = number.chars().nth(i).filter(char::is_ascii_digit) {
                    result.push(num);
                }
            }

            if let Ok(number) = result.parse::<usize>() {
                match operator {
                    Operator::Add => operator_result += number,
                    Operator::Mul => operator_result *= number,
                }
            }
        }

        sum += operator_result;
    }

    println!("Part 2 answer: {sum}");
}
