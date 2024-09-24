#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race {time, distance}
    }

    fn ways_to_beat_record(&self) -> u64 {
        let middle = self.time / 2;

        let count = (middle..=self.time).map(|time| {
            (self.time - time) * time
        }).filter(|distance| *distance > self.distance).count() as u64;

        if self.time & 0x1 == 1 {
            (count - 1) * 2
        } else {
            count * 2 - 1
        }
    }
}

fn parse_poster(input: &str) -> Vec<Race> {
    let mut race_iter = input.lines().map(|line| {
        let mut split = line.split_whitespace();
        split.next();

        let num = split.fold(String::new(), |number, digits| format!("{}{}", number, digits));
        dbg!(&num);
        num.parse().expect("Number should be parsable!")
    });

    let time = race_iter.next().expect("First line should be there!");
    let distance = race_iter.next().expect("Second line should be there!");

    vec![Race::new(time, distance)]
}


fn main() {
    let input = include_str!("../../input/input.txt");

    let races = parse_poster(input);
    let ways_prod: u64 = races.iter().map(|race| race.ways_to_beat_record()).product();

    println!("Res: {ways_prod}");
}
