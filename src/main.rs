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
    * 8 - Red Queen - (1) Must be discarded if you have (7) Time or (5) Knave of Hearts in your hand
    * 7 - Time - (1) Trade hands with another player.
    * 6 - Executioner - (2) Draw 2 cards, pick one and place 2 cards on the bottom of the deck.
    * 5 - Knave of Hearts - (2) Discard a player's hand (including your own) and make them draw a new card.
    * 4 - Nobody - (2) Protection until your next turn.
    * 3 - Tweedies - (2) Compare hands with another player. Lowest hand is out.
    * 2 - Wilkins - (2) Look at another player's hand.
    * 1 - Guard - (6) Guess a player's hand and if correct, player is out.
    * 0 - Dormouse - (2) Gain one token if no one else discarded a dormouse by the end of the round.
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

mod card;
mod logic;
mod tests;

use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};
use crate::card::Card;
use crate::logic::game_loop;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn ui_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .font(include_bytes!("../assets/ui_assets/MinimalPixel v2.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/window_background_2.png"),
                None,
            )
                .unwrap(),
        )
        .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
        .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/button_background_2.png"),
                None,
            )
                .unwrap(),
        )
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/button_hovered_background_2.png"),
                None,
            )
                .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/button_clicked_background_2.png"),
                None,
            )
                .unwrap(),
        )
        .font(include_bytes!("../assets/ui_assets/MinimalPixel v2.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(40)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/checkbox_background.png"),
                None,
            )
                .unwrap(),
        )
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/checkbox_hovered_background.png"),
                None,
            )
                .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/checkbox_clicked_background.png"),
                None,
            )
                .unwrap(),
        )
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/editbox_background.png"),
                None,
            )
                .unwrap(),
        )
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font(include_bytes!("../assets/ui_assets/MinimalPixel v2.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    let combobox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui_assets/combobox_background.png"),
                None,
            )
                .unwrap(),
        )
        .background_margin(RectOffset::new(4., 25., 6., 6.))
        .font(include_bytes!("../assets/ui_assets/MinimalPixel v2.ttf"))
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color(Color::from_rgba(210, 210, 210, 255))
        .font_size(25)
        .build();

    Skin {
        window_style,
        button_style,
        label_style,
        checkbox_style,
        editbox_style,
        combobox_style,
        ..root_ui().default_skin()
    }
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

#[macroquad::main("Love Letter RS")]
async fn main() {
    let skin = ui_skin();
    root_ui().push_skin(&skin);
    game_loop().await;
}
