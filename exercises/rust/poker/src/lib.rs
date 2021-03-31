use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq)]
struct PokerHand<'a> {
    hand: &'a str,
    cards: Vec<(u8, u8)>,
    level: PokerType,
}

impl <'a> PokerHand<'a> {
    pub fn new(hand: &'a str) -> Self {
        let cards = hand.split(" ").collect::<Vec<& str>>();

        let mut counter: HashMap<u8, u8> = HashMap::new();
        let mut suits = HashSet::new();
        cards.iter().for_each(|s| {
            let (face, suit) = s.split_at(s.len() - 1);
            let value: u8 = match face {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => face
                    .parse()
                    .unwrap_or_else(|_| panic!("\"{}\" is not a valid card face", face)),
            };
            *counter.entry(value).or_insert(0) += 1;
            suits.insert(suit);
        });
        let mut groups: Vec<(u8, u8)> = counter.drain().collect();
        groups.sort_by_key(|&(value, count)| (Reverse(count), Reverse(value)));

        // determine what rank we have
        let counts: Vec<u8> = groups.iter().map(|g| g.1).collect();
        let level = match counts[..] {
            [1, 1, 1, 1, 1] => {
                let faces: Vec<u8> = groups.iter().map(|g| g.0).collect();
                let mut result = PokerType::HighCard;
                // we may have an Ace-first straight
                if faces
                    .iter()
                    .cloned()
                    .eq((faces[faces.len() - 1]..=faces[0]).rev())
                {
                    result = PokerType::Straight;
                }

                // suit may be involved in the ranking
                if suits.len() == 1 {
                    if result == PokerType::Straight {
                        result = PokerType::StraightFlush;
                    } else {
                        result = PokerType::Flush;
                    }
                }
                result
            }
            [4, 1] => PokerType::FourOfAKind,
            [3, 2] => PokerType::FullHouse,
            [3, 1, 1] => PokerType::ThreeOfAKind,
            [2, 2, 1] => PokerType::TwoPairs,
            [2, 1, 1, 1] => PokerType::OnePair,
            _ => PokerType::HighCard,
        };

        Self {
            hand,
            cards: groups,
            level,
        }
    }
}

impl <'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.level.cmp(&other.level) {
            Ordering::Equal => Some(other.cards.cmp(&self.cards)),
            c => Some(c),
        }
    }
}

impl <'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level && self.cards == other.cards
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut my_poker: Vec<PokerHand> = hands.iter().map(|h| PokerHand::new(*h)).collect();
    my_poker.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));

    Some(my_poker
        .iter()
        .filter(|h| my_poker[0].eq(&h))
        .map(|h| h.hand)
        .collect::<Vec<&'a str>>(),
    )
}
