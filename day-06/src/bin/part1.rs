#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Race {
        Race {time, distance}
    }

    fn ways_to_beat_record(&self) -> u32 {
        let middle = self.time / 2;

        let count = (middle..=self.time).map(|time| {
            (self.time - time) * time
        }).filter(|distance| *distance > self.distance).count() as u32;

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

        split.map(|number| number.parse().expect("All values should be able to be parsed to integer!"))
    });

    let time_iter = race_iter.next().expect("First line should be there!");
    let distance_iter = race_iter.next().expect("Second line should be there!");

    time_iter.zip(distance_iter).map(|(time, distance)| Race::new(time, distance)).collect()
}


fn main() {
    let input = include_str!("../../input/input.txt");

    let races = parse_poster(input);
    let ways_prod: u32 = races.iter().map(|race| race.ways_to_beat_record()).product();

    println!("Res: {ways_prod}");
}
