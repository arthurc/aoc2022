use std::collections::HashMap;

pub fn calculate_combined_priority_for_shared_items(s: &str) -> u32 {
    s.lines()
        .flat_map(|l| Rucksack::parse(l, 2))
        .map(|r| {
            r.shared_item_occurrences()
                .keys()
                .map(|i| i.priority())
                .sum::<u32>()
        })
        .sum()
}

pub fn calculate_badge_priority_for_groups<const N: usize>(s: &str) -> u32 {
    s.lines()
        .flat_map(|l| Rucksack::parse(l, 2))
        .scan(Vec::new(), |state, x| {
            state.push(x);

            if state.len() == N {
                Some(Some(Group::new(state.drain(..).collect())))
            } else {
                Some(None)
            }
        })
        .flat_map(|o| o.into_iter())
        .flat_map(|g| g.badge().map(|b| b.priority()))
        .sum()
}

#[derive(Debug, PartialEq)]
struct Rucksack {
    compartments: Vec<Compartment>,
}
impl Rucksack {
    fn parse(s: &str, compartment_count: usize) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() || s.len() % compartment_count != 0 {
            return None;
        }

        let compartment_size = s.len() / compartment_count;

        Some(Rucksack {
            compartments: (0..compartment_count)
                .map(|i| i * compartment_size)
                .map(|i| Compartment(s[i..i + compartment_size].into()))
                .collect(),
        })
    }

    fn shared_item_occurrences(&self) -> HashMap<Item, usize> {
        self.compartments[0]
            .items()
            .filter(|i| {
                self.compartments[1..]
                    .iter()
                    .all(|compartment| compartment.contains(i))
            })
            .fold(HashMap::new(), |mut map, item| {
                *map.entry(item).or_insert(0) += 1;
                map
            })
    }

    fn item_occurrences(&self) -> HashMap<Item, usize> {
        self.compartments.iter().fold(HashMap::new(), |mut map, c| {
            for (item, _) in c.item_occurrences() {
                *map.entry(item).or_insert(0) += 1;
            }
            map
        })
    }
}

#[derive(Debug, PartialEq)]
struct Compartment(String);
impl Compartment {
    fn items(&self) -> impl Iterator<Item = Item> + '_ {
        self.0.chars().flat_map(Item::parse)
    }

    fn contains(&self, item: &Item) -> bool {
        self.items().any(|other| other == *item)
    }

    fn item_occurrences(&self) -> HashMap<Item, usize> {
        self.items().fold(HashMap::new(), |mut map, item| {
            *map.entry(item).or_insert(0) += 1;
            map
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Item(char);
impl Item {
    fn parse(c: char) -> Option<Self> {
        Some(Self(c))
    }

    fn priority(&self) -> u32 {
        if self.0.is_ascii_lowercase() {
            self.0 as u32 - 'a' as u32 + 1
        } else {
            self.0 as u32 - 'A' as u32 + 27
        }
    }
}

#[derive(Debug)]
struct Group {
    rucksacks: Vec<Rucksack>,
}
impl Group {
    fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }

    fn badge(&self) -> Option<Item> {
        self.item_occurrences()
            .into_iter()
            .find(|(_, v)| *v == self.rucksacks.len())
            .map(|(a, _)| a)
    }

    fn item_occurrences(&self) -> HashMap<Item, usize> {
        self.rucksacks.iter().fold(HashMap::new(), |mut map, r| {
            for (item, _) in r.item_occurrences() {
                *map.entry(item).or_insert(0) += 1;
            }
            map
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rucksack() {
        assert_eq!(
            Some(Rucksack {
                compartments: vec![
                    Compartment("vJrwpWtwJgWr".into()),
                    Compartment("hcsFMMfFFhFp".into())
                ]
            }),
            Rucksack::parse("vJrwpWtwJgWrhcsFMMfFFhFp", 2)
        );
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(1, Item('a').priority());
        assert_eq!(26, Item('z').priority());
        assert_eq!(27, Item('A').priority());
        assert_eq!(52, Item('Z').priority());
    }

    #[test]
    fn test_items_shared_between_compartments() {
        assert_eq!(
            vec![Item('p')],
            Rucksack {
                compartments: vec![
                    Compartment("vJrwpWtwJgWr".into()),
                    Compartment("hcsFMMfFFhFp".into())
                ]
            }
            .shared_item_occurrences()
            .keys()
            .cloned()
            .collect::<Vec<_>>()
        );

        assert_eq!(
            vec![Item('a')],
            Rucksack::parse("aaaa", 2)
                .unwrap()
                .shared_item_occurrences()
                .keys()
                .cloned()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            16u32,
            Rucksack::parse("vJrwpWtwJgWrhcsFMMfFFhFp", 2)
                .unwrap()
                .shared_item_occurrences()
                .keys()
                .map(|i| i.priority())
                .sum()
        );
        assert_eq!(
            38u32,
            Rucksack::parse("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 2)
                .unwrap()
                .shared_item_occurrences()
                .keys()
                .map(|i| i.priority())
                .sum()
        );

        assert_eq!(
            157,
            calculate_combined_priority_for_shared_items(
                "
                vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw
                "
            )
        );
    }

    #[test]
    fn test_calculate_group_badge() {
        let group1 = Group::new(
            "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            "
            .lines()
            .flat_map(|l| Rucksack::parse(l, 2))
            .collect(),
        );
        assert_eq!(Some(Item('r')), group1.badge());

        let group2 = Group::new(
            "
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "
            .lines()
            .flat_map(|l| Rucksack::parse(l, 2))
            .collect(),
        );
        assert_eq!(Some(Item('Z')), group2.badge());
    }

    #[test]
    fn test_calculate_badge_priority_for_groups_example() {
        assert_eq!(
            70,
            calculate_badge_priority_for_groups::<3>(
                "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg

            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "
            )
        );
    }
}
