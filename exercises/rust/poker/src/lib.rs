use std::cmp::Ordering;

struct PokerHand<'a> {
    hand: &'a str,
    cards: Vec<&'a str>,
}

impl <'a> PokerHand<'a> {
    pub fn new(hand: &'a str) -> Self {
        let cards = hand.split(" ").collect::<Vec<& str>>();

        Self {
            hand,
            cards,
        }
    }
}

impl <'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        None
    }
}

impl <'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter()
            .map(|card| other.cards.contains(card))
            .fold(false, |acc, curr| acc && curr)
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    // unimplemented!("Out of {:?}, which hand wins?", hands)
    hands.iter().for_each(|hand| {
        let pokerHand = PokerHand::new(hand);
        println!("//// {} ~~~~ {:?}", pokerHand.hand, pokerHand.cards);
    });

    Some(vec!())
}
