#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_possible_play() {
        let mut state = State {
            deck: create_deck(),
            discard: vec![vec![Card::Nobody], vec![Card::Nobody]],
            players: vec!["Alice".to_string(), "Bob".to_string()],
            out: Vec::new(),
            hands: vec![Some(Card::Guard), Some(Card::Guard)],
            tokens: vec![0, 0],
            round: 0,
            turn: 0,
            dormouse: None,
        };

        let card = Card::Guard;
        assert_eq!(no_possible_play(&state, &card), true);

        state.hands[0] = Some(Card::Time);
        assert_eq!(no_possible_play(&state, &card), true);

        state.hands[0] = Some(Card::Guard);
        let card = Card::Wilkins;
        assert_eq!(no_possible_play(&state, &card), true);

        let card = Card::RedQueen;
        assert_eq!(no_possible_play(&state, &card), false);
    }
}
