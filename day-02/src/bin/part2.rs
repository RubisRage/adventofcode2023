#[derive(Debug)]
#[repr(usize)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    fn from_set(set: &str) -> Vec<Cube> {
        let set = set
            .split(',')
            .map(|cube_str| cube_str.trim())
            .map(|cube_str| {
                let space = cube_str
                    .find(' ')
                    .expect("Should be a space between cube number and color");
                (&cube_str[..space], &cube_str[space + 1..])
            })
            .map(Cube::new)
            .collect();

        set
    }

    fn new((number, color): (&str, &str)) -> Self {
        let number: u32 = number.parse().expect("Should be a number");

        match color {
            "red" => Self::Red(number),
            "blue" => Self::Blue(number),
            "green" => Self::Green(number),
            _ => panic!("Should be a valid color: red, blue or green"),
        }
    }
}

#[derive(Debug)]
struct CubeGame {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

impl CubeGame {
    fn from_record(record: &str) -> Self {
        let colon_position =
            record.find(':').expect("Colon should exist, record format");
        let id = &record["Game ".len()..colon_position];

        let sets = record[colon_position + 1..]
            .split(';')
            .map(Cube::from_set)
            .collect();

        Self {
            id: id.parse().expect("Should be a number, record format"),
            sets,
        }
    }

    fn is_possible(&self, red: u32, blue: u32, green: u32) -> bool {
        let is_possible = self.sets.iter().all(|set| {
            set.iter().all(|cube| match cube {
                Cube::Blue(b) => *b <= blue,
                Cube::Red(r) => *r <= red,
                Cube::Green(g) => *g <= green,
            })
        });

        is_possible
    }

    fn power(&self) -> u32 {
        let least_cubes =
            self.sets.iter().fold([0; 3], |mut least_cubes, set| {
                for cube in set {
                    match cube {
                        Cube::Green(g) if *g > least_cubes[0] => {
                            least_cubes[0] = *g
                        }
                        Cube::Red(r) if *r > least_cubes[1] => {
                            least_cubes[1] = *r
                        }
                        Cube::Blue(b) if *b > least_cubes[2] => {
                            least_cubes[2] = *b
                        }
                        _ => continue,
                    }
                }

                least_cubes
            });

        least_cubes.iter().product()
    }
}

fn main() {
    let input = include_str!("../../input/input.txt");

    let games: Vec<_> = input.lines().map(CubeGame::from_record).collect();

    let valid_sum: u32 = games
        .iter()
        .filter(|game| game.is_possible(12, 14, 13))
        .map(|game| game.id)
        .sum();

    let power_sum: u32 = games.iter().map(|game| game.power()).sum();

    println!("{valid_sum}");
    println!("{power_sum}");
}
