/* Love Letter
    *
    * A game of Love Letter for 2-6 players.
    *
    * The goal of the game is to survive the longest.
    * Each player starts with 1 card in their hand.
    * Each round, a player draws a card and plays a card.
    * If there is more than one player at the end of the round, the player with the lowest card is out.
    * A round ends when there is only one player left or when there are no more cards in the deck.
    * The player with the most tokens at the end of the game wins.
    * Tokens are earned by winning rounds and by being the only one to discard a dormouse.
    *
    * Cards 2-6 players:
    * 9 - Alice - (1) If discarded you are out.
    * 8 - Red Queen - (1) Must be discarded if you have (6) or (7) in your hand.
    * 7 - Hatter - (2) Trade hands with another player.
    * 6 - Knave of Hearts - (2) Draw 2 cards, pick one and place 2 cards on the bottom of the deck.
    * 5 - Executioner - (2) Discard another player's hand.
    * 4 - Nobody - (2) Protection until your next turn.
    * 3 - Tweedle - (2) Compare hands with another player. The player with the lowest hand is out.
    * 2 - White Queen - (2) Look at another player's hand.
    * 1 - Guard - (5) Guess a player's hand and if correct, player is out.
    * 0 - Dormouse - (2) Earn one token at the end of the round if no other dormouse have been discarded.
    *
    * Love Letter remake by Vladimir Li
    * Original game by Seiji Kanai
    *
    * Programmed by Pontus Asp
    * 2023-02-14
    *
    * This program is free software: you can redistribute it and/or modify
    * it under the terms of the GNU General Public License as published by
    * the Free Software Foundation, either version 3 of the License, or
    * (at your option) any later version.
    *
    * This program is distributed in the hope that it will be useful,
    * but WITHOUT ANY WARRANTY; without even the implied warranty of
    * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    * GNU General Public License for more details.
    *
    * You should have received a copy of the GNU General Public License
    * along with this program.  If not, see <https://www.gnu.org/licenses/>.
    *
 */

use text_io::read;
use rand::prelude::*;

fn read() -> i32 {
    read!()
}

fn read_str() -> String {
    read!()
}

#[derive(Debug, Clone, Copy)]
enum Card {
    Alice, // 9 - (1) If discarded you are out.
    RedQueen, // 8 - (1) Must be discarded if you have (7) Time or (5) Knave of Hearts in your hand
    Time, // 7 - (1) Trade hands with another player.
    Executioner, // 6 - (2) Draw 2 cards, pick one and place 2 cards on the bottom of the deck.
    KnaveOfHearts, // 5 - (2) Discard another player's hand and make them draw a new card.
    Nobody, // 4 - (2) Protection until your next turn.
    Tweedies, // 3 - (2) Compare hands with another player. Lowest hand is out.
    Wilkins, // 2 - (2) Look at another player's hand.
    Guard, // 1 - (6) Guess a player's hand and if correct, player is out.
    Dormouse, // 0 - (2) Gain one token if no one else discarded a dormouse by the end of the round.
}


impl Card {
    fn value(&self) -> i32 {
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
    
    fn count(&self) -> i32 {
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
    
    fn name(&self) -> String {
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
            Card::KnaveOfHearts => "Discard another player's hand and make them draw a new card.".to_string(),
            Card::Nobody => "Protection until your next turn.".to_string(),
            Card::Tweedies => "Compare hands with another player. Lowest hand is out.".to_string(),
            Card::Wilkins => "Look at another player's hand.".to_string(),
            Card::Guard => "Guess a player's hand and if correct, player is out.".to_string(),
            Card::Dormouse => "Gain one token if no one else discarded a dormouse by the end of the round.".to_string(),
        }
    }

    fn to_string(&self) -> String {
        format!("{} - {} (x{}): {}", self.value(), self.name(), self.count(), self.description())
    }
}

fn create_deck() -> Vec<Card> {
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

#[derive(Debug, Clone)]
struct State {
    deck: Vec<Card>,
    discard: Vec<Vec<Card>>,
    players: Vec<String>,
    hands: Vec<Option<Card>>,
    tokens: Vec<i32>,
    round: i32,
    turn: usize,
}

fn setup() -> State {
    println!("Welcome to Love Letter!");

    let max_players: i32 = 6;
    println!("How many players are there?");
    print!(": ");
    let mut players: i32 = read();
    while players < 2 || players > max_players  {
        print!("{} is not a valid number of players. Must be between 2 and {}. Try again: ", players, max_players);
        players = read();
    }
    let players: i32 = players;
    println!("There are {} players", players);

    println!("What are your names?");
    let mut names: Vec<String> = Vec::new();
    for i in 0..players {
        print!("Player {}: ", i + 1);
        let name: String = read_str();
        names.push(name);
    }
    println!("Names: {:?}", names);
    
    let mut deck = create_deck();
    let mut discard: Vec<Vec<Card>> = Vec::new();
    for _ in 0..players {
        discard.push(Vec::new());
    }
    println!("Deck created.");
    
    // Shuffle the deck
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    println!("Deck shuffled.");
    
    // Removing one card
    println!("Removing one card.");
    deck.pop();

    // Deal cards
    println!("Dealing cards.");
    let mut hands: Vec<Card> = Vec::new();
    for _ in 0..players {
        let card = deck.pop().unwrap();
        hands.push(card);
    }
    
    // Create state
    let mut state = State {
        deck: deck,
        discard: discard,
        players: names,
        hands: hands,
        tokens: vec![0, 0, 0, 0],
        round: 1,
        turn: 0,
    };
    state
}

enum PlayError {
    InvalidCard,
    InvalidPlayer,
    InvalidGuess,
    InvalidTargetPlayer,
    InvalidTargetHand,
    InvalidHand,
}
enum RoundStatus {
    Continue,
    End,
}
type TurnResult = Result<RoundStatus, PlayError>;

fn play_target(state: State) -> usize {
    let mut target: usize = 0;
    loop {
        print!("Target player: ");
        target = read();

        // Check if target is valid
        if target < 1 || target > state.players.len() {
            println!("That is not a valid player.");
            continue;
        }

        // Check if target is out
        if state.hands[target - 1].is_none() {
            println!("That player is out.");
            continue;
        }

        // Check if target is self
        if target - 1 == state.turn {
            println!("You cannot target yourself.");
            continue;
        }

        // Check if target is protected
        if let Some(Card::Nobody) == state.hands[target - 1] {
            println!("That player is protected.");
            continue;
        }

        break;
    }

    
    target - 1
}

fn play_card(state: &mut State, card: &Card) -> TurnResult {
    if let Some(hand) = state.hands[state.turn].clone() {
        // Implement card effects
        match card {
            Card::Alice => {
                println!("You are out.");
                state.hands[state.turn] = None;
                Ok(RoundStatus::Continue)
            },
            Card::RedQueen => {
                match hand {
                    Card::Time => {
                        Err(PlayError::InvalidCard)
                    },
                    Card::Executioner => {
                        Err(PlayError::InvalidCard)
                    },
                    _ => {
                        println!("You played the Red Queen.");
                        Ok(RoundStatus::Continue)
                    }
                }
            },
            Card::Time => {
                println!("Who would you like to target?");
                let target = play_target(state);
                println!("You swap hands with {}.", state.players[target]);
                let temp = state.hands[state.turn];
                state.hands[state.turn] = state.hands[target];
                state.hands[target] = temp;
                Ok(RoundStatus::Continue)
            },
            Card::Executioner => {
                println!("You draw two cards.");
                let card1 = state.deck.pop().unwrap();
                let card2 = state.deck.pop().unwrap();
                println!("You drew:\n1. {:?}\n2. {:?}.", card1.to_string(), card2.to_string());
                println!("Which card would you like to keep?");
                let mut keep: i32 = 0;
                loop {
                    keep = read();
                    if keep == 1 || keep == 2 {
                        break;
                    } else {
                        println!("That is not a valid card.");
                    }
                }
                if keep == 1 {
                    state.hands[state.turn] = Some(card1);
                } else {
                    state.hands[state.turn] = Some(card2);
                }
                println!("You will now place the following cards at the bottom of the deck:\n1. {:?}\n2. {:?}", card1.to_string(), card2.to_string());
                println!("Which order would you like to place them in? (1 or 2):\n1. {:?} on top of {:?}\n2. {:?} on top of {:?}", card1.name(), card2.name(), card2.name(), card1.name());
                print!(": ");
                let mut order: i32 = 0;
                loop {
                    order = read();
                    if order == 1 || order == 2 {
                        break;
                    } else {
                        println!("That is not a valid order.");
                    }
                }
                
                // Place cards at bottom of deck
                if order == 1 {
                    state.deck.insert(0, card1);
                    state.deck.insert(0, card2);
                } else {
                    state.deck.insert(0, card2);
                    state.deck.insert(0, card1);
                }
                println!("You placed the cards at the bottom of the deck.");
                Ok(RoundStatus::Continue)
            },
        }
    } else {
        return Err(PlayError::InvalidHand);
    }
}

fn play_turn(state: &mut State) -> RoundStatus {
    println!("\n\n================= {} =================", state.players[state.turn]);
    println!("Round {}", state.round);
    println!("Turn {}", state.turn + 1);
    println!("Discard: {:?}", state.discard);
    println!("Tokens: {:?}", state.tokens);
    println!("{}'s turn", state.players[state.turn]);
    println!("Type 1 to draw a card.");
    let card1 = state.hands[state.turn];
    loop {
        let i: i32 = read();
        if i == 1 {
            break;
        }
    }
    let card2 = state.deck.pop().unwrap();
    println!("Your hand:\n{:?}\n{:?}", card1.to_string(), card2.to_string());
    println!("What would you like to do?");
    println!("1. Discard {:?}", card1.name());
    println!("2. Discard {:?}", card2.name());
    print!(": ");
    loop {
        let choice: i32 = read();
        let turn_result = match choice {
            1 => {
                state.hands[state.turn] = card2;
                play_card(state, &card1)
            },
            2 => {
                state.hands[state.turn] = card1;
                play_card(state, &card2)
            },
            _ => Err(PlayError::InvalidCard)
        };

        if let Ok(round_status) = turn_result {
            match round_status {
                RoundStatus::Continue => break,
                RoundStatus::End => return RoundStatus::End
            };
        } else if let Err(play_error) = turn_result {
            match play_error {
                PlayError::InvalidPlayer => {
                    println!("Can't discard if you are out of the game!");
                    break;
                },
                _ => {
                    print!("Invalid choice. Try again: ");
                }
            }
        }

    }
    
    state.turn += 1;
    if state.turn >= state.players.len() {
        state.turn = 0;
        state.round += 1;
    }
    println!("==========================================\n\n");
    RoundStatus::Continue
}

fn main() {
    let mut state = setup();
    loop {
        let round_status = play_turn(&mut state);
        match round_status {
            RoundStatus::Continue => continue,
            RoundStatus::End => break,
        }
    }
}
