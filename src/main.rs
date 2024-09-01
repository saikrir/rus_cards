use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suites = ["Clubs", "Diamond", "Spades", "Hearts"];
        let nums = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        let mut cards: Vec<String> = Vec::new();

        for s in suites {
            for c in nums {
                let card = format!("{} of {}", c, s);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    // Does in-place shuffle
    fn shuffle(&mut self) {
        let deck_size = self.cards.len();
        for i in 1..deck_size {
            let random_num = thread_rng().gen_range(0..(deck_size));
            let temp = self.cards[i].clone();
            self.cards[i] = self.cards[random_num].clone();
            self.cards[random_num] = temp;
        }
    }

    fn shuffle2(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        let len = self.cards.len();
        self.cards.split_off(len - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("My Hand {:#?}", deck.deal(5));
    println!("My Hand {:#?}", deck.cards.len());
}
