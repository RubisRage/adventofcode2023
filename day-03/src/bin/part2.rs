use itertools::{iproduct, Itertools};

fn adjacents_iter<'a>(
    grid: &'a Vec<&'a [u8]>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    const ADJACENTS: [[isize; 2]; 8] = [
        [0, 1],
        [1, 0],
        [1, 1],
        [0, -1],
        [-1, 0],
        [-1, -1],
        [1, -1],
        [-1, 1],
    ];

    ADJACENTS.iter().filter_map(move |adjacent| {
        let x = x.wrapping_add_signed(adjacent[0]);
        let y = y.wrapping_add_signed(adjacent[1]);

        if x < grid[0].len() && y < grid.len() {
            Some((x, y))
        } else {
            None
        }
    })
}

fn bound_num(
    grid: &Vec<&[u8]>,
    x: usize,
    y: usize,
) -> Option<(usize, usize, usize)> {
    if !grid[y][x].is_ascii_digit() {
        return None;
    }

    let mut num_start = x;
    let mut num_end = x + 1;

    while num_start > 0 && grid[y][num_start - 1].is_ascii_digit() {
        num_start -= 1;
    }

    while num_end < grid[y].len() && grid[y][num_end].is_ascii_digit() {
        num_end += 1
    }

    Some((y, num_start, num_end))
}

fn parse_gear_ratio(grid: &Vec<&[u8]>, x: usize, y: usize) -> Option<u32> {
    if grid[y][x] == '*' as u8 {
        let mut number_of_adjacent_nums = 0;

        let gear_ratio = adjacents_iter(&grid, x, y)
            .filter_map(|(x, y)| bound_num(grid, x, y))
            .unique()
            .filter_map(|(y, start, end)| {
                std::str::from_utf8(&grid[y][start..end])
                    .unwrap()
                    .parse::<u32>()
                    .ok()
            })
            .inspect(|_| {
                number_of_adjacent_nums += 1;
            })
            .product();

        if number_of_adjacent_nums == 2 {
            return Some(gear_ratio);
        }
    }

    None
}

fn find_gear_ratios<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter_map(move |(x, y)| parse_gear_ratio(&grid, x, y))
}

fn main() {
    let input = include_str!("../../input/input.txt");

    let gear_ratio_sum: u32 = find_gear_ratios(input).sum();
    println!("{gear_ratio_sum}");
}
