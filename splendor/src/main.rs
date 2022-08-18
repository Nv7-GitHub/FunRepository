mod board;
use board::*;

fn main() {
    let mut b = Board{
        cards: [
            [
                Card{gem: Gem::Ruby, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Emerald, count: 6}, Requirement{gem: Gem::Ruby, count: 3}], points: 4},
                Card{gem: Gem::Emerald, requirements: vec![Requirement{gem: Gem::Diamond, count: 5}, Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Ruby, count: 3}, Requirement{gem: Gem::Onyx, count: 3}], points: 3},
                Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}, Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Ruby, count: 5}, Requirement{gem: Gem::Onyx, count: 3}], points: 3},
                Card{gem: Gem::Emerald, requirements: vec![Requirement{gem: Gem::Sapphire, count: 7}], points: 4},
            ],
            [
                Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Diamond, count: 3}, Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Onyx, count: 2}], points: 1},
                Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Sapphire, count: 1}, Requirement{gem: Gem::Emerald, count: 4}, Requirement{gem: Gem::Ruby, count: 2}], points: 2},
                Card{gem: Gem::Onyx, requirements: vec![Requirement{gem: Gem::Emerald, count: 5}, Requirement{gem: Gem::Ruby, count: 3}], points: 2},
                Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Diamond, count: 5}, Requirement{gem: Gem::Sapphire, count: 3}], points: 2},
            ],
            [
                Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 3}], points: 0},
                Card{gem: Gem::Diamond, requirements: vec![Requirement{gem: Gem::Sapphire, count: 2}, Requirement{gem: Gem::Onyx, count: 2}], points: 0},
                Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Diamond, count: 1}, Requirement{gem: Gem::Emerald, count: 2}, Requirement{gem: Gem::Ruby, count: 2}], points: 0},
                Card{gem: Gem::Sapphire, requirements: vec![Requirement{gem: Gem::Emerald, count: 3}, Requirement{gem: Gem::Onyx, count: 1}], points: 0},
            ],
        ],
    };
    
    
}
