use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("../assets/day5-input.txt");

pub fn solution() {
    let mut parts = INPUT.split("\n\n");

    let Some(fresh_ranges) = parts.next() else {
        panic!("no fresh ranges")
    };
    let Some(ingredients) = parts.next() else {
        panic!("no ingredients")
    };

    let mut fresh_ranges = fresh_ranges
        .lines()
        .filter_map(|range| {
            let mut range = range.split('-');
            let from = range.next()?.parse::<usize>().ok()?;
            let to = range.next()?.parse::<usize>().ok()?;

            Some(from..=to)
        })
        .collect::<Vec<_>>();

    fresh_ranges.sort_unstable_by_key(|range| *range.start());

    let fresh_ingredients = ingredients
        .lines()
        .filter_map(|ingredient| {
            let ingredient = ingredient.parse::<usize>().ok()?;

            if fresh_ranges.iter().any(|range| range.contains(&ingredient)) {
                Some(1)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("Part 1 answer: {fresh_ingredients}");

    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    for range in fresh_ranges {
        if let Some(a) = merged_ranges
            .iter_mut()
            .find(|a| a.contains(range.start()) && !a.contains(range.end()))
        {
            *a = (*a.start())..=(*range.end());
        } else if !merged_ranges
            .iter()
            .any(|a| a.contains(range.start()) && a.contains(range.end()))
        {
            merged_ranges.push(range);
        }
    }

    println!(
        "Part 2 answer: {:?}",
        merged_ranges
            .into_iter()
            .map(Iterator::count)
            .sum::<usize>()
    );
}
