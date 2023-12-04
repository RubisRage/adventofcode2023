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

fn has_adjacent_symbol(
    grid: &Vec<&[u8]>,
    y: usize,
    start: usize,
    end: usize,
) -> bool {
    const SYMBOLS: &str = "*+#$=@/-&%";

    for current in start..end {
        for (x, y) in adjacents_iter(&grid, current, y) {
            if SYMBOLS
                .bytes()
                .find(|symbol| *symbol == grid[y][x])
                .is_some()
            {
                return true;
            }
        }
    }

    false
}

fn parse_part(grid: &Vec<&[u8]>, x: &mut usize, y: usize) -> Option<u32> {
    let line = grid[y];

    if !line[*x].is_ascii_digit() {
        *x += 1;
        return None;
    }

    let start = *x;

    while *x < line.len() && grid[y][*x].is_ascii_digit() {
        *x += 1;
    }

    if has_adjacent_symbol(&grid, y, start, *x) {
        std::str::from_utf8(&grid[y][start..*x])
            .unwrap()
            .parse()
            .ok()
    } else {
        None
    }
}

fn find_engine_parts_ids(input: &str) -> Vec<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut parts = vec![];

    for (y, line) in grid.iter().enumerate() {
        let mut x = 0usize;

        while x < line.len() {
            if let Some(part) = parse_part(&grid, &mut x, y) {
                parts.push(part);
            }
        }
    }

    parts
}

fn main() {
    let input = include_str!("../../input/input.txt");

    let parts_id_sum: u32 = find_engine_parts_ids(input).iter().sum();
    println!("{parts_id_sum}");
}
