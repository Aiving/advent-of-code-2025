pub const INPUT: &str = include_str!("../assets/day4-input.txt");

fn neighbours_count(
    input: &str,
    row: usize,
    column: usize,
    row_width: usize,
    column_height: usize,
) -> usize {
    let mut items = 0;

    for corner in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        if (corner.0 == -1 && column == 0)
            || (corner.0 == 1 && column == column_height - 1)
            || (corner.1 == -1 && row == 0)
            || (corner.1 == 1 && row == row_width - 1)
        {
            continue;
        }

        let column = (column.cast_signed() + corner.0).cast_unsigned();
        let row = (row.cast_signed() + corner.1).cast_unsigned();

        if input
            .lines()
            .nth(row)
            .and_then(|line| line.chars().nth(column))
            == Some('@')
        {
            items += 1;
        }
    }

    items
}

pub fn solution() {
    let mut input = INPUT.to_string();

    let row_width = input.lines().next().unwrap_or_default().len();
    let column_height = input.lines().count();

    let mut removed_rolls = 0usize;
    let mut is_first = true;

    loop {
        let current_input = input.clone();
        let mut rolls = 0;

        for (index, (char_idx, character)) in current_input
            .char_indices()
            .filter(|(_, c)| c != &'\n')
            .enumerate()
        {
            let row = index / row_width;
            let column = index % row_width;

            if character == '@'
                && neighbours_count(&current_input, row, column, row_width, column_height) < 4
            {
                rolls += 1;

                input.replace_range(char_idx..=char_idx, ".");
            }
        }

        if is_first {
            println!("Part 1 answer: {rolls}");

            is_first = false;
        }

        removed_rolls += rolls;

        if rolls == 0 {
            break;
        }
    }

    println!("Part 2 answer: {removed_rolls}");
}
