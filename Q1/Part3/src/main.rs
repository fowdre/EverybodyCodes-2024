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

#[derive(Clone, Copy, Debug)]
enum MonsterGroup {
    Single(Monster),
    Pair(Monster, Monster),
    Trio(Monster, Monster, Monster),
}

impl MonsterGroup {
    fn new(monsters: (Option<Monster>, Option<Monster>, Option<Monster>)) -> Option<MonsterGroup> {
        match monsters {
            (Some(first), Some(second), Some(third)) => Some(MonsterGroup::Trio(first, second, third)),

            (Some(first), Some(second), None) => Some(MonsterGroup::Pair(first, second)),
            (Some(first), None, Some(third)) => Some(MonsterGroup::Pair(first, third)),
            (None, Some(second), Some(third)) => Some(MonsterGroup::Pair(second, third)),

            (Some(first), None, None) => Some(MonsterGroup::Single(first)),
            (None, Some(second), None) => Some(MonsterGroup::Single(second)),
            (None, None, Some(third)) => Some(MonsterGroup::Single(third)),

            _ => None,
        }
    }

    fn potion_needed(&self) -> u8 {
        match self {
            MonsterGroup::Single(monster) => monster.potion_needed(),
            MonsterGroup::Pair(first, second) => first.potion_needed() + second.potion_needed() + 2,
            MonsterGroup::Trio(first, second, third) => {
                first.potion_needed() + second.potion_needed() + third.potion_needed() + 6
            }
        }
    }
}

fn letters_to_monster_group(letters: &str) -> Vec<MonsterGroup> {
    let monsters: Vec<Option<Monster>> = letters.chars().map(Monster::from_letter).collect();

    monsters.chunks_exact(3).filter_map(|chunk| MonsterGroup::new((chunk[0], chunk[1], chunk[2]))).collect()
}

fn main() -> Result<(), io::Error> {
    let file_input = read_into_string()?;
    let monster_groups = letters_to_monster_group(&file_input);
    let total_potions: u128 = monster_groups.iter().map(|group| group.potion_needed() as u128).sum();

    println!("{}", total_potions);

    Ok(())
}
