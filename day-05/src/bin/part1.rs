#[derive(Debug)]
struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl RangeMap {
    fn from_line(line: &str) -> RangeMap {
        let mut parts = line.split_whitespace();
        let destination_start = parts
            .next()
            .expect("Source start should be there, Alamanac format")
            .parse()
            .unwrap();
        let source_start = parts
            .next()
            .expect("Destination start should be there, Alamanac format")
            .parse()
            .unwrap();
        let length = parts
            .next()
            .expect("Length should be there, Alamanac format")
            .parse()
            .unwrap();

        RangeMap {
            source_start,
            destination_start,
            length,
        }
    }
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<RangeMap>,
}

impl AlmanacMap {
    fn from_input_group(group: &str) -> AlmanacMap {
        let mut lines = group.lines();
        let _ = lines.next();

        let ranges = lines.map(RangeMap::from_line).collect();

        AlmanacMap { ranges }
    }

    fn map(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if source >= range.source_start
                && source < range.source_start + range.length
            {
                return range.destination_start + source - range.source_start;
            }
        }

        source
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    map_chain: Vec<AlmanacMap>,
}

impl Almanac {
    fn min_location_number(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| {
                let mut mapped = *seed;

                for almanac_map in &self.map_chain {
                    mapped = almanac_map.map(mapped);
                }

                mapped
            })
            .min()
            .expect("There should be a lowest location")
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let mut groups = input.split("\n\n");

    let seeds = groups
        .next()
        .expect("Seeds should be there, Almanac format");

    let seeds = seeds
        .split_once(": ")
        .expect("Separator ': ' should be there, Almanac format")
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let map_chain = groups
        .map(|group| AlmanacMap::from_input_group(group))
        .collect();

    Almanac { seeds, map_chain }
}

fn main() {
    let input = include_str!("../../input/input.txt");
    let almanac = parse_almanac(input);

    println!("{}", almanac.min_location_number());
}
