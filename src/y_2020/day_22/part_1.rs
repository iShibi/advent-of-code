use std::collections::VecDeque;

pub fn solve(input_text: &String) -> usize {
	let decks = input_text
		.split("\n\n")
		.filter_map(|deck| {
			if deck.is_empty() {
				return None;
			}
			let cards = deck
				.split_terminator('\n')
				.skip(1)
				.filter_map(|card| card.parse::<usize>().ok())
				.collect::<VecDeque<_>>();
			Some(cards)
		})
		.collect::<Vec<_>>();
	let (mut player_one_deck, mut player_two_deck) = (decks[0].clone(), decks[1].clone());
	let total_cards_count = player_one_deck.len() + player_two_deck.len();
	while player_one_deck.len() > 0 && player_two_deck.len() > 0 {
		let player_one_card = player_one_deck.pop_front().unwrap();
		let player_two_card = player_two_deck.pop_front().unwrap();
		if player_one_card > player_two_card {
			player_one_deck.push_back(player_one_card);
			player_one_deck.push_back(player_two_card);
		} else {
			player_two_deck.push_back(player_two_card);
			player_two_deck.push_back(player_one_card);
		}
	}
	if player_one_deck.len() > 0 {
		let mut score = 0;
		for (idx, card) in player_one_deck.iter().enumerate() {
			score += card * (total_cards_count - idx);
		}
		return score;
	} else {
		let mut score = 0;
		for (idx, card) in player_two_deck.iter().enumerate() {
			score += card * (total_cards_count - idx);
		}
		return score;
	}
}
