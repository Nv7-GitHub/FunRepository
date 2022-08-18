mod board;
use board::*;

fn main() {
    let mut b = Board{
        cards: vec![
            vec![
                Some(Card{gem: Gem::Ruby, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Emerald, count: 6}, Requirement{gem: Gem::Ruby, count: 3}], points: 4}),
                Some(Card{gem: Gem::Emerald, requirements: vec![Requirement{gem: Gem::Diamond, count: 5}, Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Ruby, count: 3}, Requirement{gem: Gem::Onyx, count: 3}], points: 3}),
                Some(Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Ruby, count: 5}, Requirement{gem: Gem::Onyx, count: 3}], points: 3}),
                Some(Card{gem: Gem::Emerald, requirements: vec![Requirement{gem: Gem::Sapphire, count: 7}], points: 4}),
            ],
            vec![
                Some(Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Diamond, count: 3}, Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Onyx, count: 2}], points: 1}),
                Some(Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Sapphire, count: 1}, Requirement{gem: Gem::Emerald, count: 4}, Requirement{gem: Gem::Ruby, count: 2}], points: 2}),
                Some(Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Emerald, count: 5}, Requirement{gem: Gem::Ruby, count: 3}], points: 2}),
                Some(Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Diamond, count: 5}, Requirement{gem: Gem::Sapphire, count: 3}], points: 2}),
            ],
            vec![
                Some(Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}], points: 0}),
                Some(Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 2}, Requirement{gem: Gem::Onyx, count: 2}], points: 0}),
                Some(Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Diamond, count: 1}, Requirement{gem: Gem::Emerald, count: 2}, Requirement{gem: Gem::Ruby, count: 2}], points: 0}),
                Some(Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Onyx, count: 1}], points: 0}),
            ],
        ],
    };
    
    let res = b.solve(&mut Vec::new());
    println!("{res:?}");
}
