extern crate rand;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;

// 'a designates the lifetime. Because these vectors are of reference strings
// we must give a lifetime or they default to different lifetimes
// without 'a it would default to Vec<&'a str>, Vec<&'b str>, thus not allowing
// movement between the two collections as their lifetimes could end prematurely.
fn draw_card<'a>(hand: &mut Vec<&'a str>, deck: &mut Vec<&'a str>) {
    let card = deck.pop(); // returns an optional.
    match card {
        Some(c) => hand.push(c), // If card is not 'None' then push to the hand
        None => println!("Deck is out of cards!"),
    }
}

// Calculates the score of the inputted hand.
fn calculate_score(hand: &[&str]) -> i32 {
    let mut score = 0;
    let mut has_ace = false;

    for card in hand.iter() {
        // adds the result of the match expression to score.
        // if parsing card passes, just add the number.
        // else it must be either an Ace or a Face card.
        score += match card.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                // hand.iter() goes through &str,
                // so we must dereference card to compare str -> str.
                if *card == "A" {
                    has_ace = true;
                    1
                } else {
                    // Must be a face card add 10
                    10
                }
            }
        };
    }

    if has_ace && score < 12 {
        score += 10;
    }

    score
}

// Lists Player and Dealer cards and their respective scores.
fn print_status(player_cards: &[&str], dealer_cards: &[&str]) {
    println!("Player's total is {:?}", calculate_score(player_cards));
    for card in player_cards.iter() {
        print!("{} ", card);
    }
    println!();

    println!("Dealer's total is {:?}", calculate_score(dealer_cards));
    for card in dealer_cards.iter() {
        print!("{} ", card);
    }
    println!();
}

// Shuffles the deck
fn shuffle(deck: &mut [&str]) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng)
}

fn main() {
    let mut deck = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let mut player_cards: Vec<&str> = Vec::new();
    let mut dealer_cards: Vec<&str> = Vec::new();

    shuffle(&mut deck);
    println!("Dealer draws first card.");
    draw_card(&mut dealer_cards, &mut deck);

    println!("Player receives two cards.");
    draw_card(&mut player_cards, &mut deck);
    draw_card(&mut player_cards, &mut deck);
    print_status(&player_cards, &dealer_cards);

    loop {
        println!("Do you want to (H)it, (S)tay, or (Q)uit? ");

        // Gets the choice from user input
        // Pressing enter adds '\n' to the end of the input.
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // trim choice to get rid of the '\n' from the input.
        match choice.trim().to_uppercase().as_ref() {
            "H" => {
                draw_card(&mut player_cards, &mut deck);
                print_status(&player_cards, &dealer_cards);
                if calculate_score(&player_cards) > 21 {
                    println!("You busted! You lose!");
                    return;
                }
            }
            "S" => break,
            "Q" => return,
            _ => println!("Not a valid choice"),
        };
    }
    // Dealer draws until score is above 17.
    while calculate_score(&dealer_cards) < 17 {
        draw_card(&mut dealer_cards, &mut deck);
    }
    print_status(&player_cards, &dealer_cards);

    let dealer_score = calculate_score(&dealer_cards);
    if dealer_score > 21 {
        println!("The dealer busts! You Win!");
    } else {
        match dealer_score.cmp(&calculate_score(&player_cards)) {
            Ordering::Greater => println!("Dealer wins!"),
            Ordering::Less => println!("You win!"),
            Ordering::Equal => println!("It's a tie!"),
        }
    }
}

// Unit Tests for calculate score

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_score_kj() {
        let hand = vec!["K", "J"];
        let expected = 20;
        let actual = calculate_score(&hand);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_calculate_score_q7a() {
        let hand = vec!["Q", "7", "A"];
        let expected = 18;
        let actual = calculate_score(&hand);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_calculate_score_258q9() {
        let hand = vec!["2", "5", "8", "Q", "9"];
        let expected = 34;
        let actual = calculate_score(&hand);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_score_23a() {
        let hand = vec!["2", "3", "A"];
        let expected = 16;
        let actual = calculate_score(&hand);
        assert_eq!(expected, actual);
    }
}
