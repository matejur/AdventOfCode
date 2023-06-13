pub fn solve() {
    let input = include_str!("../../inputs/in21.txt");
    println!("Solving day 21");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(PartialEq, Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Item; 6] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    }, // optional to not have armor
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 7] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    }, // optional to not have a ring
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn player_wins(
    player_damage: i32,
    player_armor: i32,
    player_health: i32,
    boss_damage: i32,
    boss_armor: i32,
    boss_health: i32,
) -> bool {
    let player_damage = if player_damage > boss_armor {
        player_damage - boss_armor
    } else {
        1
    };
    let boss_damage = if boss_damage > player_armor {
        boss_damage - player_armor
    } else {
        1
    };

    let boss_dies_in = boss_health / player_damage;
    let player_dies_in = player_health / boss_damage;

    player_dies_in + 1 > boss_dies_in
}

fn part1(input: &str) -> String {
    let stats: Vec<&str> = input.split("\n").collect();
    let boss_health: i32 = stats[0].split_whitespace().last().unwrap().parse().unwrap();
    let boss_damage: i32 = stats[1].split_whitespace().last().unwrap().parse().unwrap();
    let boss_armor: i32 = stats[2].split_whitespace().last().unwrap().parse().unwrap();

    let mut best_cost = i32::MAX;
    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    // not 100% sure about this condition
                    if ring1 == ring2 && ring1.cost != 0 {
                        continue;
                    }

                    let player_damage = weapon.damage + ring1.damage + ring2.damage;
                    let player_armor = armor.armor + ring1.armor + ring2.armor;
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;

                    if cost < best_cost
                        && player_wins(
                            player_damage,
                            player_armor,
                            100,
                            boss_damage,
                            boss_armor,
                            boss_health,
                        )
                    {
                        best_cost = cost;
                    }
                }
            }
        }
    }

    best_cost.to_string()
}

fn part2(input: &str) -> String {
    let stats: Vec<&str> = input.split("\n").collect();
    let boss_health: i32 = stats[0].split_whitespace().last().unwrap().parse().unwrap();
    let boss_damage: i32 = stats[1].split_whitespace().last().unwrap().parse().unwrap();
    let boss_armor: i32 = stats[2].split_whitespace().last().unwrap().parse().unwrap();

    let mut best_cost = 0;
    for weapon in WEAPONS {
        for armor in ARMOR {
            for ring1 in RINGS {
                for ring2 in RINGS {
                    // not 100% sure about this condition
                    if ring1 == ring2 && ring1.cost != 0 {
                        continue;
                    }

                    let player_damage = weapon.damage + ring1.damage + ring2.damage;
                    let player_armor = armor.armor + ring1.armor + ring2.armor;
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;

                    if cost > best_cost
                        && !player_wins(
                            player_damage,
                            player_armor,
                            100,
                            boss_damage,
                            boss_armor,
                            boss_health,
                        )
                    {
                        best_cost = cost;
                    }
                }
            }
        }
    }

    best_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::player_wins;

    #[test]
    fn day21_duel() {
        assert!(player_wins(5, 5, 8, 7, 2, 12));
    }
}
