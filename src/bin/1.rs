use std::io;

use aoc_2016::*;

fn flatten<'a>(input: impl Iterator<Item = &'a str> + 'a) -> impl Iterator<Item = char> + 'a {
    input.flat_map(|direction| {
        let (heading, distance) = (
            direction[0..=0].chars().next().unwrap(),
            direction[1..].parse::<i32>().unwrap(),
        );
        std::iter::once(heading).chain((1..distance).map(|_| 'S'))
    })
}

fn matcher(heading: char, direction: char) -> (char, i32, i32) {
    match (heading, direction) {
        ('N', 'R') => ('E', 1, 0),
        ('N', 'L') => ('W', -1, 0),
        ('N', 'S') => ('N', 0, 1),
        ('S', 'R') => ('W', -1, 0),
        ('S', 'L') => ('E', 1, 0),
        ('S', 'S') => ('S', 0, -1),
        ('E', 'R') => ('S', 0, -1),
        ('E', 'L') => ('N', 0, 1),
        ('E', 'S') => ('E', 1, 0),
        ('W', 'R') => ('N', 0, 1),
        ('W', 'L') => ('S', 0, -1),
        ('W', 'S') => ('W', -1, 0),
        _ => panic!("Invalid heading"),
    }
}

fn p1(input: &Vec<String>) -> i32 {
    let r = flatten(input[0].split(", ")).fold(('N', 0, 0), |(heading, x, y), direction| {
        let (h, dx, dy) = matcher(heading, direction);
        (h, x + dx, y + dy)
    });
    ((r.1 + r.2) as i32).abs()
}

fn p2(input: &Vec<String>) -> i32 {
    let path: Vec<(i32, i32)> = flatten(input[0].split(", "))
        .scan(('N', 0, 0), |state, direction| {
            let (heading, x, y) = *state;
            let (h, dx, dy) = matcher(heading, direction);
            *state = (h, x + dx, y + dy);
            Some((x + dx, y + dy))
        })
        .collect();
    (0..path.len())
        .find_map(|i| {
            (i + 1..path.len()).find_map(|j| {
                if path[i] == path[j] {
                    Some((path[i].0 + path[i].1).abs())
                } else {
                    None
                }
            })
        })
        .unwrap_or(0)
}

fn main() -> io::Result<()> {
    let input = read_in()?;
    write_out(p1(&input));
    write_out(p2(&input));
    Ok(())
}
