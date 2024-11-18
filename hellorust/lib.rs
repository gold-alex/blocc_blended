#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use fluentbase_sdk::{
    basic_entrypoint,
    derive::{function_id, router, Contract},
    SharedAPI, U256,
};

#[derive(Contract)]
struct MiniBaccarat<SDK: SharedAPI> {
    sdk: SDK,
}

pub trait MiniBaccaratAPI {
    fn startGame(&mut self) -> String;
    fn getResult(&mut self) -> String;
    fn drawCard(&mut self) -> u8;
}

const DECK_POSITION_KEY: U256 = U256::from_limbs([1, 0, 0, 0]);
const GAME_RESULT_KEY: U256 = U256::from_limbs([2, 0, 0, 0]);

// Baccarat card values: 1-9, with 0 representing 10 and face cards
const BACCARAT_DECK_VALUES: [u8; 52] = [
    // Hearts
    1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0,
    // Diamonds
    1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0,
    // Clubs
    1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0,
    // Spades
    1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0,
];

const NUMBER_OF_DECKS: usize = 6;

// Build the shoe by repeating the deck
fn build_shoe() -> Vec<u8> {
    let mut shoe = Vec::with_capacity(BACCARAT_DECK_VALUES.len() * NUMBER_OF_DECKS);
    for _ in 0..NUMBER_OF_DECKS {
        shoe.extend_from_slice(&BACCARAT_DECK_VALUES);
    }
    shoe
}

#[router(mode = "solidity")]
impl<SDK: SharedAPI> MiniBaccaratAPI for MiniBaccarat<SDK> {
    #[function_id("startGame()")]
    fn startGame(&mut self) -> String {
        
        // Initialize variables
        let mut player_cards = Vec::new();
        let mut banker_cards = Vec::new();

        // Deal initial cards using drawCard function
        player_cards.push(self.drawCard());
        banker_cards.push(self.drawCard());
        player_cards.push(self.drawCard());
        banker_cards.push(self.drawCard());

        let mut player_total = (player_cards[0] + player_cards[1]) % 10;
        let mut banker_total = (banker_cards[0] + banker_cards[1]) % 10;

        // Check for naturals
        if player_total >= 8 || banker_total >= 8 {
            // Both stand
        } else {
            // Player's third card rule
            let mut player_third_card_value = None;
            if player_total <= 5 {
                // Player draws third card
                let card = self.drawCard();
                player_cards.push(card);
                player_third_card_value = Some(card);
                player_total = (player_total + card) % 10;
            }

            // Banker's third card rule
            if let Some(player_third_card_value) = player_third_card_value {
                // Player drew third card
                match banker_total {
                    0..=2 => {
                        // Banker draws
                        let card = self.drawCard();
                        banker_cards.push(card);
                        banker_total = (banker_total + card) % 10;
                    }
                    3 => {
                        if player_third_card_value != 8 {
                            let card = self.drawCard();
                            banker_cards.push(card);
                            banker_total = (banker_total + card) % 10;
                        }
                    }
                    4 => {
                        if (2..=7).contains(&player_third_card_value) {
                            let card = self.drawCard();
                            banker_cards.push(card);
                            banker_total = (banker_total + card) % 10;
                        }
                    }
                    5 => {
                        if (4..=7).contains(&player_third_card_value) {
                            let card = self.drawCard();
                            banker_cards.push(card);
                            banker_total = (banker_total + card) % 10;
                        }
                    }
                    6 => {
                        if (6..=7).contains(&player_third_card_value) {
                            let card = self.drawCard();
                            banker_cards.push(card);
                            banker_total = (banker_total + card) % 10;
                        }
                    }
                    _ => {} // Banker stands on 7
                }
            } else {
                // Player did not draw third card
                if banker_total <= 5 {
                    // Banker draws third card
                    let card = self.drawCard();
                    banker_cards.push(card);
                    banker_total = (banker_total + card) % 10;
                }
            }
        }

        // Determine the winner
        let result = if player_total > banker_total {
            1 // Player wins
        } else if banker_total > player_total {
            2 // Banker wins
        } else {
            3 // Tie
        };

        // Store result
        self.sdk
            .write_storage(GAME_RESULT_KEY, U256::from_limbs([result as u64, 0, 0, 0]));

        "GAME STARTED".to_string()
    }

    #[function_id("getResult()")]
    fn getResult(&mut self) -> String {
        let result = self.sdk.storage(&GAME_RESULT_KEY);
        match u256_to_u32(&result) {
            1 => "Player wins".to_string(),
            2 => "Banker wins".to_string(),
            3 => "Tie".to_string(),
            _ => "No game played".to_string(),
        }
    }

    #[function_id("drawCard()")]
    fn drawCard(&mut self) -> u8 {
        let mut deck_position = self.get_storage_u32(&DECK_POSITION_KEY, 0) as usize;

        // Build the shoe if not already built
        let shoe_size = BACCARAT_DECK_VALUES.len() * NUMBER_OF_DECKS;

        // Reshuffle if deck is exhausted
        if deck_position >= shoe_size {
            deck_position = 0;
        }

        // Get card from the shoe
        let card = get_shoe_card(deck_position);

        deck_position += 1;

        // Update the deck position in storage
        self.sdk
            .write_storage(DECK_POSITION_KEY, U256::from_limbs([deck_position as u64, 0, 0, 0]));

        card
    }
}

impl<SDK: SharedAPI> MiniBaccarat<SDK> {
    fn deploy(&self) {
        // Custom deployment logic if needed
    }

    fn get_storage_u32(&self, key: &U256, default: u32) -> u32 {
        let value = self.sdk.storage(key);
        if value.is_zero() {
            default
        } else {
            u256_to_u32(&value)
        }
    }
}

// Helper function to get card from the shoe
fn get_shoe_card(deck_position: usize) -> u8 {
    let shoe = build_shoe();
    shoe[deck_position % shoe.len()]
}

// Helper function to convert U256 to u32
fn u256_to_u32(u: &U256) -> u32 {
    (u.as_limbs()[0] & 0xFFFFFFFF) as u32
}

basic_entrypoint!(MiniBaccarat);