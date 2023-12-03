
fn generate_adjacents(grid: &Vec<&[u8]>, x: usize, y: usize) -> Vec<(usize, usize)> {
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

    let mut valid_adjacents = vec![];

    for adjacent in ADJACENTS.iter() {
        let x = x.wrapping_add_signed(adjacent[0]); 
        let y = y.wrapping_add_signed(adjacent[1]); 
        let is_valid = x < grid[0].len() && y < grid.len();

        if is_valid {
            valid_adjacents.push((x, y));
        }
    }

    valid_adjacents
}

fn has_adjacent_symbol(grid: &Vec<&[u8]>, y: usize, start: usize, end: usize) -> bool {
    const SYMBOLS: &[u8] = "*+#$=@/-&%".as_bytes();

    for current in start..end {
        for (x, y) in generate_adjacents(&grid, current, y) {
            if SYMBOLS.iter().find(|symbol| **symbol == grid[y][x]).is_some() {
                return true;
            }
        }
    }

    false
}

fn parse_part(grid: &Vec<&[u8]>, mut x: usize, y: usize) -> (usize, Option<u32>) {
    let line = grid[y];

    if !line[x].is_ascii_digit() {
        return (x+1, None);
    }

    let start = x;
    let mut end = x+1;

    while end < line.len() && grid[y][end].is_ascii_digit() {
        end += 1;
    }

    x = end;

    if has_adjacent_symbol(&grid, y, start, end) {
        (x, std::str::from_utf8(&grid[y][start..end]).unwrap().parse().ok())
    } else {
        (x, None)
    }
}

fn find_engine_parts_ids(input: &str) -> Vec<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut parts = vec![];

    for (y, line) in grid.iter().enumerate() {
        let mut x = 0usize;

        while x < line.len() {
            let (new_x, part) = parse_part(&grid, x, y);
            x = new_x;

            if let Some(part) = part {
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
