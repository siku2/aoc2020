use std::collections::{HashSet, VecDeque};

type Card = u8;

type Deck = VecDeque<Card>;

fn parse_deck(s: &str) -> Option<Deck> {
    s.trim()
        .lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().ok())
        .collect()
}

fn calculate_deck_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * card as usize)
        .sum()
}

type Players = (Deck, Deck);

fn parse_players(s: &str) -> Option<Players> {
    let mut it = s.trim().split(':');
    it.next()?;
    let player_a = parse_deck(it.next()?)?;
    let player_b = parse_deck(it.next()?)?;
    Some((player_a, player_b))
}

type PlayerOneWins = bool;

fn play_until_winner((deck_a, deck_b): &mut Players) -> Option<PlayerOneWins> {
    loop {
        let card_a = deck_a.pop_front()?;
        let card_b = deck_b.pop_front()?;
        if card_a > card_b {
            deck_a.push_back(card_a);
            deck_a.push_back(card_b);
        } else {
            deck_b.push_back(card_b);
            deck_b.push_back(card_a);
        }

        if deck_a.is_empty() {
            return Some(false);
        } else if deck_b.is_empty() {
            return Some(true);
        }
    }
}

fn first_part(players: &mut Players) -> Option<usize> {
    let player_one_wins = play_until_winner(players)?;
    if player_one_wins {
        Some(calculate_deck_score(&players.0))
    } else {
        Some(calculate_deck_score(&players.1))
    }
}

#[must_use]
fn recursive_round(players: &mut Players) -> Option<()> {
    let (deck_a, deck_b) = players;
    let card_a = deck_a.pop_front()?;
    let card_b = deck_b.pop_front()?;

    let a_wins = if deck_a.len() >= card_a as usize && deck_b.len() >= card_b as usize {
        let sub_deck_a = deck_a.iter().copied().take(card_a as usize).collect();
        let sub_deck_b = deck_b.iter().copied().take(card_b as usize).collect();
        recursive_game(&mut (sub_deck_a, sub_deck_b))?
    } else {
        card_a > card_b
    };

    if a_wins {
        deck_a.push_back(card_a);
        deck_a.push_back(card_b);
    } else {
        deck_b.push_back(card_b);
        deck_b.push_back(card_a);
    }

    Some(())
}

fn recursive_game(players: &mut Players) -> Option<PlayerOneWins> {
    let mut previous_states = HashSet::new();
    loop {
        previous_states.insert(players.clone());
        recursive_round(players)?;

        let (deck_a, deck_b) = players;
        if deck_a.is_empty() {
            return Some(false);
        } else if deck_b.is_empty() {
            return Some(true);
        }

        if previous_states.contains(players) {
            return Some(true);
        }
    }
}

fn second_part(players: &mut Players) -> Option<usize> {
    if recursive_game(players)? {
        Some(calculate_deck_score(&players.0))
    } else {
        Some(calculate_deck_score(&players.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        Player 1:
        9
        2
        6
        3
        1
        
        Player 2:
        5
        8
        4
        7
        10
    "#;

    #[test]
    fn first() {
        let mut players = parse_players(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(first_part(&mut players).expect("failed to solve"), 306);
    }
    #[test]
    fn second() {
        let mut players = parse_players(EXAMPLE_INPUT).expect("failed to parse input");
        assert_eq!(second_part(&mut players).expect("failed to solve"), 291);
    }
}
