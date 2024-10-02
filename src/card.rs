#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Card {
    Alice, // 9 - (1) If discarded you are out.
    RedQueen, // 8 - (1) Must be discarded if you have (7) Time or (5) Knave of Hearts in your hand
    Time, // 7 - (1) Trade hands with another player.
    Executioner, // 6 - (2) Draw 2 cards, pick one and place 2 cards on the bottom of the deck.
    KnaveOfHearts, // 5 - (2) Discard a player's hand (including your own) and make them draw a new card.
    Nobody, // 4 - (2) Protection until your next turn.
    Tweedies, // 3 - (2) Compare hands with another player. Lowest hand is out.
    Wilkins, // 2 - (2) Look at another player's hand.
    Guard, // 1 - (6) Guess a player's hand and if correct, player is out.
    Dormouse, // 0 - (2) Gain one token if no one else discarded a dormouse by the end of the round.
}


impl Card {
    pub(crate) fn value(&self) -> usize {
        match self {
            Card::Alice => 9,
            Card::RedQueen => 8,
            Card::Time => 7,
            Card::Executioner => 6,
            Card::KnaveOfHearts => 5,
            Card::Nobody => 4,
            Card::Tweedies => 3,
            Card::Wilkins => 2,
            Card::Guard => 1,
            Card::Dormouse => 0,
        }
    }

    pub(crate) fn count(&self) -> usize {
        match self {
            Card::Alice => 1,
            Card::RedQueen => 1,
            Card::Time => 1,
            Card::KnaveOfHearts => 2,
            Card::Executioner => 2,
            Card::Nobody => 2,
            Card::Tweedies => 2,
            Card::Wilkins => 2,
            Card::Guard => 6,
            Card::Dormouse => 2,
        }
    }

    pub(crate) fn name(&self) -> String {
        match self {
            Card::Alice => "Alice".to_string(),
            Card::RedQueen => "The Red Queen".to_string(),
            Card::Time => "Time".to_string(),
            Card::Executioner => "Executioner".to_string(),
            Card::KnaveOfHearts => "Knave of Hearts".to_string(),
            Card::Nobody => "Nobody".to_string(),
            Card::Tweedies => "The Tweedies".to_string(),
            Card::Wilkins => "Wilkins".to_string(),
            Card::Guard => "Guard".to_string(),
            Card::Dormouse => "The Dormouse".to_string(),
        }
    }

    fn description(&self) -> String {
        match self {
            Card::Alice => "If discarded you are out.".to_string(),
            Card::RedQueen => "Must be discarded if you have (6) or (7) in your hand.".to_string(),
            Card::Time => "Trade hands with another player.".to_string(),
            Card::Executioner => "Draw 2 cards, pick one and place 2 cards on the bottom of the deck.".to_string(),
            Card::KnaveOfHearts => "Discard a player's hand (including your own) and make them draw a new card.".to_string(),
            Card::Nobody => "Protection until your next turn.".to_string(),
            Card::Tweedies => "Compare hands with another player. Lowest hand is out.".to_string(),
            Card::Wilkins => "Look at another player's hand.".to_string(),
            Card::Guard => "Guess a player's hand and if correct, player is out.".to_string(),
            Card::Dormouse => "Gain one token if no one else discarded a dormouse by the end of the round.".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} - {} (x{}): {}", self.value(), self.name(), self.count(), self.description())
    }

    pub(crate) fn targetting(&self) -> bool {
        match self {
            Card::Alice => false,
            Card::RedQueen => false,
            Card::Time => true,
            Card::Executioner => false,
            Card::KnaveOfHearts => true,
            Card::Nobody => false,
            Card::Tweedies => true,
            Card::Wilkins => true,
            Card::Guard => true,
            Card::Dormouse => false,
        }
    }

    pub(crate) fn can_target_self(&self) -> bool {
        match self {
            Card::KnaveOfHearts => true,
            _ => false,
        }
    }

    pub(crate) fn protects(&self) -> bool {
        match self {
            Card::Nobody => true,
            _ => false,
        }
    }
}

pub fn list_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    cards.push(Card::Alice);
    cards.push(Card::RedQueen);
    cards.push(Card::Time);
    cards.push(Card::Executioner);
    cards.push(Card::KnaveOfHearts);
    cards.push(Card::Nobody);
    cards.push(Card::Tweedies);
    cards.push(Card::Wilkins);
    cards.push(Card::Guard);
    cards.push(Card::Dormouse);
    cards
}

pub fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for _ in 0..Card::Alice.count() {
        deck.push(Card::Alice);
    }
    for _ in 0..Card::RedQueen.count() {
        deck.push(Card::RedQueen);
    }
    for _ in 0..Card::Time.count() {
        deck.push(Card::Time);
    }
    for _ in 0..Card::Executioner.count() {
        deck.push(Card::Executioner);
    }
    for _ in 0..Card::KnaveOfHearts.count() {
        deck.push(Card::KnaveOfHearts);
    }
    for _ in 0..Card::Nobody.count() {
        deck.push(Card::Nobody);
    }
    for _ in 0..Card::Tweedies.count() {
        deck.push(Card::Tweedies);
    }
    for _ in 0..Card::Wilkins.count() {
        deck.push(Card::Wilkins);
    }
    for _ in 0..Card::Guard.count() {
        deck.push(Card::Guard);
    }
    for _ in 0..Card::Dormouse.count() {
        deck.push(Card::Dormouse);
    }
    deck
}

