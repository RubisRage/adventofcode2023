struct ScratchCard<'a> {
    winning: Vec<&'a str>,
    numbers: Vec<&'a str>,
}

use std::collections::VecDeque;

fn scratch_card_cloning(input: &str) -> u32 {
    let table: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, card) = line
                .split_once(": ")
                .expect("The separator ': ' should exist, card format");

            let (winning, numbers) = card
                .split_once('|')
                .expect("The separator ': ' should exist, card format");

            ScratchCard {
                winning: Vec::from_iter(
                    winning.split(' ').filter(|w| !w.is_empty()),
                ),
                numbers: Vec::from_iter(
                    numbers.split(' ').filter(|n| !n.is_empty()),
                ),
            }
        })
        .collect();

    let mut to_process = VecDeque::from_iter(0..table.len());
    let mut processed = 0;

    while let Some(card_index) = to_process.pop_front() {
        let card = &table[card_index];
        let mut number_of_wins = 0;

        for number in card.numbers.iter() {
            if card.winning.contains(number) {
                number_of_wins += 1;
            }
        }

        to_process.extend(card_index + 1..=card_index + number_of_wins);
        processed += 1;
    }

    processed
}

fn main() {
    let input = include_str!("../../input/input.txt");

    let number_of_cards = scratch_card_cloning(input);

    println!("{number_of_cards}");
}
