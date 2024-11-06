use std::io::{self, BufRead};

fn read_into_string() -> Result<String, io::Error> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

#[derive(Debug)]
enum Monster {
    Ant,
    Beetle,
    Cockroach,
}

impl Monster {
    fn from_letter(letter: char) -> Option<Monster> {
        match letter {
            'A' => Some(Monster::Ant),
            'B' => Some(Monster::Beetle),
            'C' => Some(Monster::Cockroach),
            _ => None,
        }
    }

    fn get_potion_number_to_defeat(&self) -> u32 {
        match self {
            Monster::Ant => 0,
            Monster::Beetle => 1,
            Monster::Cockroach => 3,
        }
    }
}

fn letters_to_monsters(letters: &str) -> Vec<Monster> {
    letters.chars().filter_map(Monster::from_letter).collect()
}

fn main() -> Result<(), io::Error> {
    let file_input = read_into_string()?;
    let monsters = letters_to_monsters(&file_input);
    let total_potions: u128 = monsters.iter().map(|monster| monster.get_potion_number_to_defeat() as u128).sum();

    println!("{}", total_potions);

    Ok(())
}
