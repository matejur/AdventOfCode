pub fn solve() {
    let input = include_str!("../../inputs/in22.txt");
    println!("Solving day 22");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn least_mana(
    player_turn: bool,
    mut boss_health: i32,
    boss_damage: i32,
    mut player_health: i32,
    mut player_mana: i32,
    mana_spent: i32,
    mut shield_effect: i32,
    mut poison_effect: i32,
    mut recharge_effect: i32,
    best_mana: &mut i32,
    part1: bool,
) {
    if !part1 && player_turn {
        player_health -= 1;
    }

    if player_health <= 0 || player_mana <= 0 || mana_spent > *best_mana {
        return;
    }

    if boss_health <= 0 {
        *best_mana = mana_spent;
        return;
    }
    let player_armor = if shield_effect > 0 { 7 } else { 0 };

    if poison_effect > 0 {
        boss_health -= 3;
    }

    if recharge_effect > 0 {
        player_mana += 101;
    }

    shield_effect -= 1;
    poison_effect -= 1;
    recharge_effect -= 1;

    if !player_turn {
        let damage = if boss_damage > player_armor {
            boss_damage - player_armor
        } else {
            1
        };
        least_mana(
            true,
            boss_health,
            boss_damage,
            player_health - damage,
            player_mana,
            mana_spent,
            shield_effect,
            poison_effect,
            recharge_effect,
            best_mana,
            part1,
        );
    } else {
        // Magic missile
        least_mana(
            false,
            boss_health - 4,
            boss_damage,
            player_health,
            player_mana - 53,
            mana_spent + 53,
            shield_effect,
            poison_effect,
            recharge_effect,
            best_mana,
            part1,
        );

        // Drain
        least_mana(
            false,
            boss_health - 2,
            boss_damage,
            player_health + 2,
            player_mana - 73,
            mana_spent + 73,
            shield_effect,
            poison_effect,
            recharge_effect,
            best_mana,
            part1,
        );

        // Shield
        if shield_effect <= 0 {
            least_mana(
                false,
                boss_health,
                boss_damage,
                player_health,
                player_mana - 113,
                mana_spent + 113,
                6,
                poison_effect,
                recharge_effect,
                best_mana,
                part1,
            );
        }

        // Poison
        if poison_effect <= 0 {
            least_mana(
                false,
                boss_health,
                boss_damage,
                player_health,
                player_mana - 173,
                mana_spent + 173,
                shield_effect,
                6,
                recharge_effect,
                best_mana,
                part1,
            );
        }

        // Recharge
        if recharge_effect <= 0 {
            least_mana(
                false,
                boss_health,
                boss_damage,
                player_health,
                player_mana - 229,
                mana_spent + 229,
                shield_effect,
                poison_effect,
                5,
                best_mana,
                part1,
            );
        }
    }
}

fn part1(input: &str) -> String {
    let stats: Vec<&str> = input.split("\n").collect();
    let boss_health: i32 = stats[0].split_whitespace().last().unwrap().parse().unwrap();
    let boss_damage: i32 = stats[1].split_whitespace().last().unwrap().parse().unwrap();

    let mut best_mana = i32::MAX;
    least_mana(
        true,
        boss_health,
        boss_damage,
        50,
        500,
        0,
        0,
        0,
        0,
        &mut best_mana,
        true,
    );

    best_mana.to_string()
}

fn part2(input: &str) -> String {
    let stats: Vec<&str> = input.split("\n").collect();
    let boss_health: i32 = stats[0].split_whitespace().last().unwrap().parse().unwrap();
    let boss_damage: i32 = stats[1].split_whitespace().last().unwrap().parse().unwrap();

    let mut best_mana = i32::MAX;
    least_mana(
        true,
        boss_health,
        boss_damage,
        50,
        500,
        0,
        0,
        0,
        0,
        &mut best_mana,
        false,
    );

    best_mana.to_string()
}
