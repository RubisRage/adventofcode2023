fn parse_points(input: &str) -> Vec<u32> {
    let mut cards_points = vec![];

    for card in input.lines() {
        let (_, card) = card
            .split_once(": ")
            .expect("There should be ':', card format");

        let (winning_numbers, card_numbers) = card
            .split_once('|')
            .expect("There should be '|', card format");

        let mut card_points = 0;

        for number in card_numbers.split(' ').filter(|n| !n.is_empty()) {
            if winning_numbers
                .split(' ')
                .filter(|w| !w.is_empty())
                .find(|winning_number| *winning_number == number)
                .is_some()
            {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }

        cards_points.push(card_points);
    }

    cards_points
}

fn main() {
    let input = include_str!("../../input/input.txt");

    let points: u32 = parse_points(input).iter().sum();

    println!("{points}");
}
