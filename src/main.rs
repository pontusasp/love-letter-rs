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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn value(&self) -> usize {
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
    
    fn count(&self) -> usize {
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

fn list_cards() -> Vec<Card> {
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
    out: Vec<usize>,
    hands: Vec<Option<Card>>,
    tokens: Vec<i32>,
    round: i32,
    turn: usize,
    dormouse: Option<usize>,
}

fn setup(last_state: Option<State>) -> State {
    let (mut deck, mut discard, mut players, out, mut hands, mut tokens, round, turn, dormouse) = match last_state {
        Some(state) => (create_deck(), Vec::new(), state.players, Vec::new(), Vec::new(), state.tokens, state.round, 0, None::<usize>),
        None => (create_deck(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 1, 0, None),
    };

    if last_state.is_none() {
        println!("Welcome to Love Letter!");

        let max_players: i32 = 6;
        println!("How many players are there?");
        print!(": ");
        let player_count: i32 = read!();
        while player_count < 2 || player_count > max_players  {
            print!("{} is not a valid number of players. Must be between 2 and {}. Try again: ", player_count, max_players);
            player_count = read!();
        }
        println!("There are {} players", player_count);

        println!("What are your names?");
        for i in 0..player_count {
            print!("Player {}: ", i + 1);
            let name: String = read!();
            players.push(name);
        }
        println!("Names: {:?}", players);
        
        // Assign tokens
        for _ in 0..player_count {
            tokens.push(0);
        }
    }
    
    for _ in 0..players.len() {
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
    for _ in 0..players.len() {
        let card = deck.pop().unwrap();
        hands.push(Some(card));
    }
    
    // Create state
    let state = State {
        deck: deck,
        discard: discard,
        players: players,
        out: out,
        hands: hands,
        tokens: tokens,
        round: round,
        turn: turn,
        dormouse: dormouse,
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
    Retry,
    TieBreaker,
    End,
}
type TurnResult = Result<RoundStatus, PlayError>;

fn play_target(state: State) -> Option<usize> {
    println!("Who would you like to target?");

    // List players
    println!("0: Cancel.");
    for i in 0..state.players.len() {
        if state.hands[i].is_some() {
            println!("{}: {}", i + 1, state.players[i]);
        }
    }

    let mut target: usize;
    loop {
        print!("Target player: ");
        target = read!();

        // Check if user cancelled
        if target == 0 {
            return None;
        }

        // Check if target is valid
        if target < 1 || target > state.players.len() {
            println!("That is not a valid player.");
            continue;
        }

        // Check if target is out
        if state.out.contains(&target) {
            println!("That player is out.");
            continue;
        }

        // Check if target is self
        if target - 1 == state.turn {
            println!("You cannot target yourself.");
            continue;
        }

        // Check if target is protected
        if Some(&Card::Nobody) == state.discard[target - 1].last() {
            println!("That player is protected.");
            continue;
        }

        break;
    }

    
    Some(target - 1)
}

fn play_card(state_: State, card: Card) -> (TurnResult, State) {
    let mut state = state_.clone();
    let result = {
        if let Some(hand) = state.hands[state.turn].clone() {
            // Discard card
            state.discard[state.turn].push(card);

            // Clone state for targetting
            let state_clone = state.clone();

            // Implement card effects
            match card {
                Card::Alice => {
                    println!("You are out.");
                    state.out.push(state.turn);
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
                    let target = play_target(state_clone);
                    if target.is_none() {
                        return (Ok(RoundStatus::Retry), state_);
                    }
                    let target = target.unwrap();
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
                    let mut keep: i32;
                    loop {
                        keep = read!();
                        if keep == 1 || keep == 2 {
                            break;
                        } else {
                            println!("That is not a valid card.");
                        }
                    }
                    let (hand, card) = {
                        let hand = state.hands[state.turn].unwrap();
                        if keep == 1 {
                            state.hands[state.turn] = Some(card1);
                            (hand, card2)
                        } else {
                            state.hands[state.turn] = Some(card2);
                            (hand, card1)
                        }
                    };
                    println!("You will now place the following cards at the bottom of the deck:\n1. {:?}\n2. {:?}", hand.to_string(), card.to_string());
                    println!("Which order would you like to place them in? (1 or 2):\n1. {:?} on top of {:?}\n2. {:?} on top of {:?}", hand.name(), card.name(), card.name(), hand.name());
                    print!(": ");
                    let mut order: i32;
                    loop {
                        order = read!();
                        if order == 1 || order == 2 {
                            break;
                        } else {
                            println!("That is not a valid order.");
                        }
                    }
                    
                    // Place cards at bottom of deck
                    if order == 1 {
                        state.deck.insert(0, hand);
                        state.deck.insert(0, card);
                    } else {
                        state.deck.insert(0, card);
                        state.deck.insert(0, hand);
                    }
                    println!("You placed the cards at the bottom of the deck.");
                    Ok(RoundStatus::Continue)
                },
                Card::KnaveOfHearts => {
                    let target = play_target(state_clone);
                    if target.is_none() {
                        return (Ok(RoundStatus::Retry), state_);
                    }
                    let target = target.unwrap();
                    
                    // Makes target discard a card and draw a new one
                    println!("{} discards {}.", state.players[target], state.hands[target].unwrap().name());
                    let hand = state.hands[target].unwrap();
                    state.discard[target].push(hand);
                    state.hands[target] = None;
                    if state.discard[target].last().unwrap() == &Card::Alice {
                        println!("{} is out.", state.players[target]);
                        state.out.push(target);
                    } else {
                        let card = state.deck.pop().unwrap();
                        println!("{} draws a card.", state.players[target]);
                        state.hands[target] = Some(card);
                    }
                    Ok(RoundStatus::Continue)
                },
                Card::Nobody => {
                    println!("You are protected.");
                    Ok(RoundStatus::Continue)
                },
                Card::Tweedies => {
                    let target = play_target(state_clone);
                    if target.is_none() {
                        return (Ok(RoundStatus::Retry), state_);
                    }
                    let target = target.unwrap();
                    println!("You compare hands with {}.", state.players[target]);
                    let your_score = state.hands[state.turn].unwrap().value();
                    let their_score = state.hands[target].unwrap().value();
                    if your_score > their_score {
                        println!("You win.");
                        state.discard[target].push(state.hands[target].unwrap());
                        state.hands[target] = None;
                    } else if your_score < their_score {
                        println!("You lose.");
                        state.discard[state.turn].push(state.hands[state.turn].unwrap());
                        state.hands[state.turn] = None;
                    } else {
                        println!("You tie.");
                    }
                    Ok(RoundStatus::Continue)
                },
                Card::Wilkins => {
                    let target = play_target(state_clone);
                    if target.is_none() {
                        return (Ok(RoundStatus::Retry), state_);
                    }
                    let target = target.unwrap();
                    println!("{}'s hand is:\n{}", state.players[target], state.hands[target].unwrap().to_string()); 
                    Ok(RoundStatus::Continue)
                },
                Card::Guard => {
                    let target = play_target(state_clone);
                    if target.is_none() {
                        return (Ok(RoundStatus::Retry), state_);
                    }
                    let target = target.unwrap();
                    println!("What card would you like to guess?");
                    
                    // List all cards except the Guard
                    let card_list = list_cards();
                    let cards = card_list.len();
                    for card in card_list {
                        if card != Card::Guard {
                            println!("{}. {:?}", card.value(), card.to_string());
                        }
                    }

                    // Get guess
                    print!(": ");
                    let mut guess: usize;
                    loop {
                        guess = read!();
                        if guess != Card::Guard.value() && guess < cards.try_into().unwrap() {
                            break;
                        } else {
                            println!("That is not a valid card.");
                            print!(": ");
                        }
                    }
                    let guess = list_cards()[(cards - guess - 1) as usize];
                    if state.hands[target].unwrap() == guess {
                        println!("You guessed {:?} correctly.", guess.name());
                        state.discard[target].push(state.hands[target].unwrap());
                        state.hands[target] = None;
                    } else {
                        println!("You guessed {:?}, which is incorrect.", guess.name());
                    }
                    Ok(RoundStatus::Continue)
                },
                Card::Dormouse => {
                    if let Some(player) = state.dormouse {
                        state.dormouse = None;
                        println!("You discarded the second Dormouse, nor you or {} will be awarded a token.", state.players[player]);
                    } else {
                        state.dormouse = Some(state.turn);
                        println!("You discarded the first Dormouse.");
                    }
                    Ok(RoundStatus::Continue)
                }
            }
        } else {
            Err(PlayError::InvalidHand)
        }
    };

    (result, state)
}

fn play_turn(state_: State) -> (RoundStatus, State) {
    // Check if game is over
    if state_.out.len() == state_.players.len() - 1 {
        return (RoundStatus::End, state_);
    }

    // Check if player is out
    if state_.out.contains(&state_.turn) {
        return (RoundStatus::Continue, state_);
    }

    // Check if there is cards in the deck
    if state_.deck.len() == 0 {
        return (RoundStatus::TieBreaker, state_);
    }
    
    // Play turn
    let mut state = state_.clone();
    println!("\n\n================= {} =================", state.players[state.turn]);
    println!("Round {}", state.round);
    println!("Turn {}", state.turn + 1);
    println!("Discard: {:?}", state.discard);
    println!("Tokens: {:?}", state.tokens);
    println!("{}'s turn", state.players[state.turn]);
    println!("Type 1 to draw a card.");
    let card1 = state.hands[state.turn].unwrap();
    loop {
        let i: i32 = read!();
        if i == 1 {
            break;
        }
    }
    let card2 = state.deck.pop().unwrap();
    println!("Your hand:\n{:?}\n{:?}", card1.to_string(), card2.to_string());
    println!("What would you like to do?");
    println!("1. Discard/Play {:?}", card1.name());
    println!("2. Discard/Play {:?}", card2.name());
    print!(": ");
    loop {
        let choice: i32 = read!();
        let turn_result = match choice {
            1 => {
                state.hands[state.turn] = Some(card2);
                play_card(state, card1)
            },
            2 => {
                state.hands[state.turn] = Some(card1);
                play_card(state, card2)
            },
            _ => (Err(PlayError::InvalidCard), state),
        };
        state = turn_result.1;

        if let Ok(round_status) = turn_result.0 {
            match round_status {
                RoundStatus::Continue => break,
                RoundStatus::Retry => return play_turn(state_),
                RoundStatus::End => return (RoundStatus::End, state),
            };
        } else if let Err(play_error) = turn_result.0 {
            match play_error {
                PlayError::InvalidPlayer => {
                    println!("Oops, can't discard if you are out of the game!");
                    break;
                },
                _ => {
                    print!("Invalid choice. Try again.");
                    return play_turn(state_);
                }
            }
        }

    }

    println!("==========================================\n\n");
    (RoundStatus::Continue, state)
}

fn play_tie_breaker(state_: State) -> State {
    println!("Tie breaker!");
    let mut state = state_.clone();

    // Check the hands of all players that are not out
    let mut hands = Vec::new();
    for i in 0..state.players.len() {
        if !state.out.contains(&i) {
            hands.push(state.hands[i].unwrap());
        }
    }

    // Check what card has the highest value
    let mut highest_card = hands[0];
    for card in hands {
        if card.value() > highest_card.value() {
            highest_card = card;
        }
    }

    // All players with hands lower than the highest card are out
    for i in 0..state.players.len() {
        if !state.out.contains(&i) {
            if state.hands[i].unwrap().value() < highest_card.value() {
                state.out.push(i);
            }
        }
    }

    state
}

fn main() {
    let mut state = setup(None);
    loop {
        loop {
            let round_status = play_turn(state);
            state = round_status.1;
            
            state.turn += 1;
            if state.turn >= state.players.len() {
                state.turn = 0;
            }
            
            match round_status.0 {
                RoundStatus::Continue => continue,
                RoundStatus::Retry => {
                    println!("Invalid choice. Try again.");
                    continue;
                },
                RoundStatus::TieBreaker => {
                    println!("Tie breaker!");
                    state = play_tie_breaker(state);
                    break;
                },
                RoundStatus::End => break,
            }
        }

        // Check what players won
        for i in 0..state.players.len() {
            if !state.out.contains(&i) {
                state.tokens[i] += 1;
                println!("{} got a token and now has {} of them!", state.players[i], state.tokens[i])
            }
        }

        // Check if there is a Dormouse
        if let Some(player) = state.dormouse {
            state.tokens[player] += 1;
            println!("{} was the only one to discard a Dormouse and is awarded a token and now has {} of them!", state.players[player], state.tokens[player])
        }

        // Print leaderboard
        let mut leaderboard: Vec<(usize, i32)> = Vec::new();
        for i in 0..state.players.len() {
            leaderboard.push((i, state.tokens[i]));
        }
        leaderboard.sort_by(|a, b| b.1.cmp(&a.1));
        println!("Leaderboard:");
        for (i, player) in leaderboard.iter().enumerate() {
            println!("{}. {} with {} tokens", i + 1, state.players[player.0], player.1);
        }

        println!("Would you like to play another round?\n1. Yes\n2. No");
        loop {
            print!(": ");
            let choice: i32 = read!();
            match choice {
                1 => {
                    state = setup(Some(state));
                    break;
                },
                2 => return,
                _ => {
                    println!("Invalid choice. Try again.");
                    continue;
                }
            }
        }
    }
}
