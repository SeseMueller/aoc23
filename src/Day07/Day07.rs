use std::collections::HashMap;
use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Hand {
    cards: [char; 5],
    bid: i32,
}

// Impl ord for hand
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // frequencies of cards
        let mut freqs_self = HashMap::new();
        let mut freqs_other = HashMap::new();
        for card in self.cards.iter() {
            *freqs_self.entry(card).or_insert(0) += 1;
        }

        for card in other.cards.iter() {
            *freqs_other.entry(card).or_insert(0) += 1;
        }

        let mut freqs_self = freqs_self
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<Vec<(i32, &char)>>();

        let mut freqs_other = freqs_other
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<Vec<(i32, &char)>>();

        freqs_self.sort_by(|a, b| a.0.cmp(&b.0));
        freqs_self.reverse();
        freqs_other.sort_by(|a, b| a.0.cmp(&b.0));
        freqs_other.reverse();

        if freqs_self[0].0 > freqs_other[0].0 {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 < freqs_other[0].0 {
            return Some(std::cmp::Ordering::Less);
        }

        // If self has full house and other has 3 of a kind, self wins
        if freqs_self[0].0 == 3
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 3
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 3
            && freqs_self[1].0 == 1
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Less);
        } else if freqs_self[0].0 == 2
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Less);
        }

        // two pair beat one pair
        if freqs_self[0].0 == 2
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 2
            && freqs_self[1].0 == 1
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Less);
        }

        // Otherwise the frequencies are the same.
        if freqs_self.iter().map(|x| x.0).collect::<Vec<_>>()
            != freqs_other.iter().map(|x| x.0).collect::<Vec<_>>()
        {
            panic!("Something went wrong: {:?} {:?}", freqs_self, freqs_other);
        }

        let letter_to_value = |c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).expect("Error converting to value: {c}") as i32,
        };

        let self_values = self
            .cards
            .iter()
            .map(|x| letter_to_value(*x))
            .collect::<Vec<_>>();
    
        let other_values = other
            .cards
            .iter()
            .map(|x| letter_to_value(*x))
            .collect::<Vec<_>>();

        // Compare elementwise
        for (s, o) in self_values.iter().zip(other_values.iter()) {
            if s > o {
                return Some(std::cmp::Ordering::Greater);
            } else if s < o {
                return Some(std::cmp::Ordering::Less);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand2 {
    cards: [char; 5],
    bid: i32,
}

impl std::cmp::Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Error comparing hands")
    }
}

// Impl ord for hand2
impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        // Now the fun begins
        // J are jokers, so they can be anything. Find the maximum this hand can be
        // by replacing Js with As, Ks, Qs, etc.

        let self_j_replace = self.cards.iter().filter(|x| **x != 'J').map(|c| c.clone()).unique().chain(std::iter::once('A')).collect::<Vec<_>>();

        let self_possibilities = self.cards.iter().map(|x| match x {
            'J' => self_j_replace.clone(), 
            _ => Vec::from([*x]),
        });

        let other_j_replace = other.cards.iter().filter(|x| **x != 'J').map(|c| c.clone()).unique().chain(std::iter::once('A')).collect::<Vec<_>>();

        let other_possibilities = other.cards.iter().map(|x| match x {
            'J' => other_j_replace.clone(),
            _ => Vec::from([*x]),
        });

        // dbg!(&self_possibilities.clone().collect::<Vec<_>>());
        // dbg!(&other_possibilities.clone().collect::<Vec<_>>());

        let self_max = self_possibilities
            .multi_cartesian_product()
            .map(|x| Hand2 {cards: x.into_iter().collect::<Vec<_>>().try_into().expect(""), bid: self.bid})
            .max().expect("Error finding max");

        let other_max = other_possibilities
            .multi_cartesian_product()
            .map(|x| Hand2 {cards: x.into_iter().collect::<Vec<_>>().try_into().expect(""), bid: other.bid})
            .max().expect("Error finding max");

        // // DEBUG
        // println!("self: {:?} self_max: {:?}", self, self_max);
        // println!("other: {:?} other_max: {:?}", other, other_max);

        // frequencies of cards
        let mut freqs_self = HashMap::new();
        let mut freqs_other = HashMap::new();
        for card in self_max.cards.iter() {
            *freqs_self.entry(card).or_insert(0) += 1;
        }

        for card in other_max.cards.iter() {
            *freqs_other.entry(card).or_insert(0) += 1;
        }

        let mut freqs_self = freqs_self
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<Vec<(i32, &char)>>();

        let mut freqs_other = freqs_other
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<Vec<(i32, &char)>>();

        freqs_self.sort_by(|a, b| a.0.cmp(&b.0));
        freqs_self.reverse();
        freqs_other.sort_by(|a, b| a.0.cmp(&b.0));
        freqs_other.reverse();

        if freqs_self[0].0 > freqs_other[0].0 {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 < freqs_other[0].0 {
            return Some(std::cmp::Ordering::Less);
        }

        // If self has full house and other has 3 of a kind, self wins
        if freqs_self[0].0 == 3
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 3
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 3
            && freqs_self[1].0 == 1
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Less);
        } else if freqs_self[0].0 == 2
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 3
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Less);
        }

        // two pair beat one pair
        if freqs_self[0].0 == 2
            && freqs_self[1].0 == 2
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 1
        {
            return Some(std::cmp::Ordering::Greater);
        } else if freqs_self[0].0 == 2
            && freqs_self[1].0 == 1
            && freqs_other[0].0 == 2
            && freqs_other[1].0 == 2
        {
            return Some(std::cmp::Ordering::Less);
        }

        // Otherwise the frequencies are the same.
        if freqs_self.iter().map(|x| x.0).collect::<Vec<_>>()
            != freqs_other.iter().map(|x| x.0).collect::<Vec<_>>()
        {
            panic!("Something went wrong: {:?} {:?}", freqs_self, freqs_other);
        }

        let letter_to_value = |c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0, // This is the only difference
            'T' => 10,
            _ => c.to_digit(10).expect("Error converting to value: {c}") as i32,
        };

        let self_values = self
            .cards
            .iter()
            .map(|x| letter_to_value(*x))
            .collect::<Vec<_>>();
    
        let other_values = other
            .cards
            .iter()
            .map(|x| letter_to_value(*x))
            .collect::<Vec<_>>();

        // Compare elementwise
        for (s, o) in self_values.iter().zip(other_values.iter()) {
            if s > o {
                return Some(std::cmp::Ordering::Greater);
            } else if s < o {
                return Some(std::cmp::Ordering::Less);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");

    let mut hands = text
        .lines()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
        .map(|x| Hand {
            cards: x.0.chars().collect::<Vec<_>>().try_into().expect(""),
            bid: x.1,
        })
        .collect::<Vec<_>>();

    // Sort hands
    hands.sort();

    let values = hands.iter().enumerate().map(|(i, x)| x.bid * (i as i32 + 1)).sum::<i32>();
    
    println!("values: {}", values);
    // Part 2

    let mut hands2 = text
        .lines()
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
        .map(|x| Hand2 {
            cards: x.0.chars().collect::<Vec<_>>().try_into().expect(""),
            bid: x.1,
        })
        .collect::<Vec<_>>();

    // Sort hands
    hands2.sort();

    let values2 = hands2.iter().enumerate().map(|(i, x)| x.bid * (i as i32 + 1)).sum::<i32>();

    println!("values2: {}", values2);




    // Test: is Hand2::partial_cmp correct?
    // Compare T55J5 to T4444
    let hand1 = Hand2 {cards: ['T', '5', '5', 'J', '5'], bid: 1};
    let hand2 = Hand2 {cards: ['T', '4', '4', '4', '4'], bid: 1};
    assert!(hand1 > hand2);
}
