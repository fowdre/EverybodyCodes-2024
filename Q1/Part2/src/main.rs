use std::io;

fn read_into_string() -> Result<String, io::Error> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Monster {
    Ant,
    Beetle,
    Cockroach,
    Dragonfly,
}

impl Monster {
    fn from_letter(letter: char) -> Option<Monster> {
        match letter {
            'A' => Some(Monster::Ant),
            'B' => Some(Monster::Beetle),
            'C' => Some(Monster::Cockroach),
            'D' => Some(Monster::Dragonfly),
            _ => None,
        }
    }

    fn potion_needed(&self) -> u8 {
        match self {
            Monster::Ant => 0,
            Monster::Beetle => 1,
            Monster::Cockroach => 3,
            Monster::Dragonfly => 5,
        }
    }
}

fn letters_to_monster_pairs(letters: &str) -> Vec<(Option<Monster>, Option<Monster>)> {
    let monsters: Vec<Option<Monster>> = letters.chars().map(Monster::from_letter).collect();

    monsters.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect()
}

fn get_potion_needed(monster_pair: (Option<Monster>, Option<Monster>)) -> u32 {
    match monster_pair {
        (Some(first), Some(second)) => (first.potion_needed() as u32 + 1) + (second.potion_needed() as u32 + 1),
        (Some(first), None) => first.potion_needed() as u32,
        (None, Some(second)) => second.potion_needed() as u32,
        (None, None) => 0,
    }
}

fn main() -> Result<(), io::Error> {
    let file_input = read_into_string()?;
    let monster_pairs = letters_to_monster_pairs(&file_input);
    let total_potions: u128 = monster_pairs.iter().map(|pair| get_potion_needed(*pair) as u128).sum();

    println!("{}", total_potions);

    Ok(())
}
