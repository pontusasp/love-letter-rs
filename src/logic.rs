use async_recursion::async_recursion;
use macroquad::hash;
use macroquad::rand::ChooseRandom;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin, widgets};
use text_io::read;
use crate::{clear_screen, State};
use crate::card::{Card, create_deck, list_cards};

const MARGIN: f32 = 20.0;

fn no_possible_play(state: &State, card: &Card) -> bool {
    let hand = state.hands[state.turn].unwrap();
    let card_targetting = card.targetting() && !card.can_target_self();
    let hand_targetting = hand.targetting() && !hand.can_target_self();
    if card_targetting && hand_targetting {
        // Check if all players are protected
        let mut all_protected = true;
        for i in 0..state.players.len() {
            if !state.out.contains(&i) && (state.discard[i].len() == 0 || !state.discard[i].last().unwrap().protects()) && i != state.turn {
                all_protected = false;
                break;
            }
        }
        all_protected
    } else {
        false
    }
}

fn play_target(state: State, card: &Card) -> Target {
    // Check if both cards player could play needs a target that is not self
    let all_protected = no_possible_play(&state, card);

    if all_protected {
        println!("All players are protected.");
    } else {
        println!("Who would you like to target?");
    }

    // List players
    println!("0: Cancel.");
    if all_protected == false {
        for i in 0..state.players.len() {
            if state.hands[i].is_some() {
                println!("{}: {}", i + 1, state.players[i]);
            }
        }
    } else {
        println!("1: Discard without any target.");
        loop {
            print!(": ");
            let target = read!();
            match target {
                0 => return Target::Cancel,
                1 => return Target::None,
                _ => {
                    println!("That is not a valid option.");
                    continue;
                }
            }
        }
    }

    let mut target: usize;
    loop {
        print!("Target player: ");
        target = read!();

        // Check if user cancelled
        if target == 0 {
            return Target::Cancel;
        }

        // Check if target is valid
        if target < 1 || target > state.players.len() {
            println!("That is not a valid player.");
            continue;
        }

        // Check if target is out
        if state.out.contains(&(target - 1)) {
            println!("That player is out.");
            continue;
        }

        // Check if target is self
        if target - 1 == state.turn && card.can_target_self() == false {
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


    Target::Player(target - 1)
}

fn play_card(state_: State, card: Card) -> (TurnResult, State) {
    if state_.hands[state_.turn].is_none() {
        return (Err(PlayError::InvalidHand), state_);
    }
    if state_.out.contains(&state_.turn) {
        return (Err(PlayError::InvalidPlayer), state_);
    }
    let mut state = state_.clone();
    let result = {
        if let Some(hand) = state.hands[state.turn].clone() {
            // Discard card
            state.discard[state.turn].push(card);

            // Clone state for targetting
            let state_clone = state.clone();

            match hand {
                Card::RedQueen => {
                    match card {
                        Card::Time => {
                            return (Err(PlayError::InvalidCard), state_);
                        },
                        Card::Executioner => {
                            return (Err(PlayError::InvalidCard), state_);
                        },
                        _ => {}
                    };
                },
                _ => {},
            }


            // Implement card effects
            match card {
                Card::Alice => {
                    println!("You are out.");
                    state.out.push(state.turn);
                    Ok(RoundStatus::Continue)
                },
                Card::RedQueen => {
                    println!("You played the Red Queen.");
                    Ok(RoundStatus::Continue)
                },
                Card::Time => {
                    let target = play_target(state_clone, &card);
                    match target {
                        Target::Cancel => return (Ok(RoundStatus::Retry), state_),
                        Target::None => {
                            println!("You discard without a target.");
                            Ok(RoundStatus::Continue)
                        },
                        Target::Player(target) => {
                            println!("You swap hands with {}.", state.players[target]);
                            let temp = state.hands[state.turn];
                            state.hands[state.turn] = state.hands[target];
                            state.hands[target] = temp;
                            Ok(RoundStatus::Continue)
                        },
                    }
                },
                Card::Executioner => {
                    println!("You draw two cards.");
                    let card1 = state.deck.pop().unwrap();
                    let card2 = state.deck.pop().unwrap();
                    println!("You drew:\n1. {}\n2. {}", card1.to_string(), card2.to_string());
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
                    println!("You will now place the following cards at the bottom of the deck:\n1. {}\n2. {}", hand.to_string(), card.to_string());
                    println!("Which order would you like to place them in? (1 or 2):\n1. \"{}\" on top of \"{}\"\n2. \"{}\" on top of \"{}\"", hand.name(), card.name(), card.name(), hand.name());
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
                    let target = play_target(state_clone, &card);
                    match target {
                        Target::Cancel => return (Ok(RoundStatus::Retry), state_),
                        Target::None => return (Err(PlayError::InvalidTargetPlayer), state_), // This should never happen
                        Target::Player(target) => {
                            // Makes target discard a card and draw a new one
                            println!("{} discards {}.", state.players[target], state.hands[target].unwrap().name());
                            let hand = state.hands[target].unwrap();
                            state.discard[target].push(hand);
                            state.hands[target] = None;
                            if state.discard[target].last().unwrap() == &Card::Alice {
                                println!("{} is out.", state.players[target]);
                                state.out.push(target);
                            } else {
                                let card = state.deck.pop();
                                if let Some(card) = card {
                                    println!("{} draws a card.", state.players[target]);
                                    state.hands[target] = Some(card);
                                } else {
                                    println!("{} has no cards left to draw and is out.", state.players[target]);
                                    state.out.push(target);
                                }
                            }
                            Ok(RoundStatus::Continue)
                        },
                    }
                },
                Card::Nobody => {
                    println!("You are protected.");
                    Ok(RoundStatus::Continue)
                },
                Card::Tweedies => {
                    let target = play_target(state_clone, &card);
                    match target {
                        Target::Cancel => return (Ok(RoundStatus::Retry), state_),
                        Target::None => {
                            println!("You discard without a target.");
                            Ok(RoundStatus::Continue)
                        },
                        Target::Player(target) => {
                            println!("You compare hands with {} who has a {}.", state.players[target], state.hands[target].unwrap().name());
                            let your_score = state.hands[state.turn].unwrap().value();
                            let their_score = state.hands[target].unwrap().value();
                            if your_score > their_score {
                                println!("You win, {} is out.", state.players[target]);
                                state.out.push(target);
                            } else if your_score < their_score {
                                println!("You lose, you are out.");
                                state.out.push(state.turn);
                            } else {
                                println!("You tie.");
                            }
                            Ok(RoundStatus::Continue)
                        },
                    }
                },
                Card::Wilkins => {
                    let target = play_target(state_clone, &card);
                    match target {
                        Target::Cancel => return (Ok(RoundStatus::Retry), state_),
                        Target::None => {
                            println!("You discard without a target.");
                            Ok(RoundStatus::Continue)
                        },
                        Target::Player(target) => {
                            println!("{}'s hand is:\n{}", state.players[target], state.hands[target].unwrap().to_string());
                            Ok(RoundStatus::Continue)
                        },
                    }
                },
                Card::Guard => {
                    let target = play_target(state_clone, &card);
                    match target {
                        Target::Cancel => return (Ok(RoundStatus::Retry), state_),
                        Target::None => {
                            println!("You discard without a target.");
                            Ok(RoundStatus::Continue)
                        },
                        Target::Player(target) => {
                            println!("What card would you like to guess?");

                            // List all cards except the Guard
                            let card_list = list_cards();
                            let cards = card_list.len();
                            for card in card_list {
                                if card != Card::Guard {
                                    println!("{}. {}", card.value(), card.to_string());
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
                                println!("You guessed \"{}\" correctly, {} is out.", guess.name(), state.players[target]);
                                state.out.push(target);
                            } else {
                                println!("You guessed \"{}\", which is incorrect.", guess.name());
                            }
                            Ok(RoundStatus::Continue)
                        },
                    }
                },
                Card::Dormouse => {
                    match state.dormouse {
                        Some(player) if player == state.turn => {
                            println!("You discarded the second Dormouse.");
                        },
                        Some(player) => {
                            println!("You discarded the second Dormouse, nor you or {} will be awarded a token.", state.players[player]);
                            state.dormouse = None;
                        },
                        None => {
                            println!("You discarded the first Dormouse.");
                            state.dormouse = Some(state.turn);
                        },
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

#[async_recursion]
async fn play_turn(state_: State) -> (RoundStatus, State) {
    clear_screen();

    // Check if game is over
    if state_.out.len() == state_.players.len() - 1 {
        return (RoundStatus::End, state_);
    }

    // Check if player is out
    if state_.out.contains(&state_.turn) {
        return (RoundStatus::Skip, state_);
    }

    // Check if there is cards in the deck
    if state_.deck.len() == 0 {
        return (RoundStatus::TieBreaker, state_);
    }

    let mut stage = 0;
    let mut state = state_.clone();
    let mut card1 = None;
    let mut card2 = None;
    let mut choice: Option<u32> = None;

    while stage < 2 {
        clear_background(BLACK);
        root_ui().window(hash!(), vec2(MARGIN, MARGIN), vec2(screen_width() - MARGIN * 2.0, screen_height() - MARGIN * 2.0), |ui| {
            ui.label(None, &format!("================= {} =================", state.players[state.turn]));
            ui.label(None, &format!("{}'s turn", state.players[state.turn]));
            ui.label(None, &format!("Round {}", state.round));
            ui.label(None, &format!("Discard piles:"));

            for (i, player) in state.players.iter().enumerate() {
                let mut player_text = String::new();
                if state.discard[i].len() == 0 {
                    if state.out.contains(&i) {
                        player_text += &format!("\t{}. {} (Out): ", i + 1, player);
                    } else {
                        player_text += &format!("\t{}. {}: ", i + 1, player);
                    }
                } else {
                    if state.out.contains(&i) {
                        player_text += &format!("\t{}. {} (Out): {}", i + 1, player, state.discard[i][0].name());
                    } else {
                        player_text += &format!("\t{}. {}: {}", i + 1, player, state.discard[i][0].name());
                    }
                    for card in state.discard[i].iter().skip(1) {
                        player_text += &format!(", {}", card.name());
                    }
                }
                ui.label(None, &player_text);
            }
            let mut tokens = format!("Tokens: {} {}", state.players[0], state.tokens[0]);
            for (i, player) in state.players.iter().skip(1).enumerate() {
                tokens += &format!(", {} {}", player, state.tokens[i+1]);
            }
            tokens.push('.');
            ui.label(None, &tokens);


            match stage {
                0 => {
                    if ui.button(None, "Draw Card") {
                        card1 = Some(state.hands[state.turn].unwrap());
                        card2 = Some(state.deck.pop().unwrap());
                        stage += 1;
                    }
                }
                1 => {
                    let card1 = &card1.unwrap();
                    let card2 = &card2.unwrap();
                    ui.label(None, "What would you like to do?");
                    if ui.button(None, format!("1. Discard/Play: {}", card1.to_string()).as_str()) {
                        choice = Some(1);
                        stage += 1;
                    }
                    if ui.button(None, format!("2. Discard/Play: {}", card2.to_string()).as_str()) {
                        choice = Some(2);
                        stage += 1;
                    }
                }
                _ => ()
            }
        });

        if let Some(choice) = choice {
            let turn_result = match choice {
                1 => {
                    state.hands[state.turn] = card2;
                    play_card(state, card1.unwrap())
                },
                2 => {
                    state.hands[state.turn] = card1;
                    play_card(state, card2.unwrap())
                },
                _ => (Err(PlayError::InvalidCard), state),
            };
            state = turn_result.1;

            if let Ok(round_status) = turn_result.0 {
                match round_status {
                    RoundStatus::Continue => break,
                    RoundStatus::Skip => return (RoundStatus::Skip, state_),
                    RoundStatus::Retry => return play_turn(state_).await,
                    RoundStatus::TieBreaker => return (RoundStatus::TieBreaker, state),
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
                        return play_turn(state_).await;
                    }
                }
            }
            println!("==========================================\n\n");
        }
        next_frame().await;
    }
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

async fn setup(last_state: Option<State>) -> State {
    let (mut deck, mut discard, mut players, out, mut hands, mut tokens, round, turn, dormouse) = match last_state {
        Some(ref state) => (create_deck(), Vec::new(), state.players.clone(), Vec::new(), Vec::new(), state.tokens.clone(), state.round.clone(), 0, None::<usize>),
        None => (create_deck(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), 1, 0, None),
    };

    if last_state.is_none() {
        let mut player_count = 2;
        let mut setting_up = true;
        while setting_up {
            clear_background(BLACK);
            const MAX_PLAYERS: u32 = 6;
            let width = screen_width();
            let height = screen_height();
            root_ui().window(hash!(), vec2(MARGIN, MARGIN), vec2(width - MARGIN * 2., height - MARGIN * 2.), |ui| {
                ui.label(None, "Welcome to Love Letter!");
                ui.label(None, "");
                ui.label(None, "How many players are there? (2-6)");
                ui.label(None, "");
                ui.label(None, &format!("{player_count}"));
                if ui.button(None, "+") {
                    player_count += 1;
                }
                ui.same_line(50.0);
                if ui.button(None, "-") {
                    player_count -= 1;
                }
                player_count = 2.max(player_count).min(MAX_PLAYERS);

                if ui.button(None, "Next") {
                    setting_up = false;
                }
            });
            next_frame().await;
        }
        println!("There are {} players", player_count);
        let mut setting_names = true;
        players = vec![String::new(); player_count as usize];
        while setting_names {
            clear_background(BLACK);

            let width = screen_width();
            let height = screen_height();

            root_ui().window(hash!(), vec2(MARGIN, MARGIN), vec2(width - MARGIN * 2., height - MARGIN * 2.), |ui| {
                ui.label(None, "What are your names?");
                for i in 0..player_count as usize {
                    ui.input_text(hash!() + i as u64, &format!("Player {}", i + 1), &mut players[i])
                }
                if ui.button(None, "Start") {
                    setting_names = false;
                }
            });

            next_frame().await;
        }

        // Assign tokens
        for _ in 0..player_count {
            tokens.push(0);
        }
    } else {
        // Rotate the players
        let last_player = players[0].clone();
        players.remove(0);
        players.push(last_player);

        let last_token = tokens[0].clone();
        tokens.remove(0);
        tokens.push(last_token);
    }

    for _ in 0..players.len() {
        discard.push(Vec::new());
    }
    println!("Deck created.");

    // Shuffle the deck
    deck.shuffle();
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

    print!("\nPlayers: {}", players[0]);
    for i in 1..players.len() {
        print!(", {}", players[i]);
    }
    println!(".");

    // Create state
    let state = State {
        deck,
        discard,
        players,
        out,
        hands,
        tokens,
        round,
        turn,
        dormouse,
    };

    state
}

enum PlayError {
    InvalidCard,
    InvalidPlayer,
    InvalidTargetPlayer,
    InvalidHand,
}
enum RoundStatus {
    Continue,
    Skip,
    Retry,
    TieBreaker,
    End,
}
type TurnResult = Result<RoundStatus, PlayError>;

enum Target {
    Player(usize),
    None,
    Cancel,
}

enum Scene {
    SetupScene,
    PlayScene,
    GameOverScene,
}

pub async fn game_loop() {
    let mut state = None;
    let mut scene = Scene::SetupScene;
    loop {
        scene = match scene {
            Scene::SetupScene => {
                state = Some(setup(None).await);
                Scene::PlayScene
            }
            Scene::PlayScene => {
                state = Some(play_scene(state).await);
                Scene::GameOverScene
            }
            Scene::GameOverScene => {
                state = game_over_scene(state).await;
                if state.is_some() {
                    Scene::PlayScene
                } else {
                    Scene::SetupScene
                }
            }
        }
    }
}

async fn play_scene(state: Option<State>) -> State {
    let mut state = state.unwrap();
    loop {
        clear_background(BLACK);
        let round_status = play_turn(state).await;
        state = round_status.1;

        state.turn += 1;
        if state.turn >= state.players.len() {
            state.turn = 0;
        }

        next_frame().await;
        match round_status.0 {
            RoundStatus::Continue => {
                continue;
            },
            RoundStatus::Skip => {
                continue;
            },
            RoundStatus::Retry => {
                panic!("Retry should not be returned from play_turn");
            },
            RoundStatus::TieBreaker => {
                state = play_tie_breaker(state);
                break;
            },
            RoundStatus::End => break,
        }
    }
    state
}

async fn game_over_scene(state: Option<State>) -> Option<State> {
    let mut state = state?;
    check_win_conditions(&mut state);
    print_leaderboard(&state);

    play_again_query(state).await
}

fn check_win_conditions(state: &mut State) {
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
}

fn print_leaderboard(state: &State) {
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
}

async fn play_again_query(state: State) -> Option<State> {
    println!("Would you like to play another round?\n1. Yes\n2. No");
    loop {
        print!(": ");
        let choice: i32 = read!();
        match choice {
            1 => {
                return Some(setup(Some(state)).await);
            },
            2 => return None,
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            }
        }
    }
}