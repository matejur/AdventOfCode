pub fn solve() {
    let input = include_str!("../../inputs/in18.txt");
    println!("Solving day 18");
    println!("Part1: {}", part1(input, 100));
    println!("Part2: {}", part2(input, 100));
}

fn get_active_neighbors(grid: &Vec<Vec<bool>>, x: i32, y: i32) -> i32 {
    let mut active = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }

            if grid[ny as usize][nx as usize] {
                active += 1;
            }
        }
    }

    active
}

fn grid_step(grid: Vec<Vec<bool>>, corners_stay: bool) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; grid[0].len()]; grid.len()];

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            let active_neighbors = get_active_neighbors(&grid, x as i32, y as i32);
            if grid[y][x] {
                if active_neighbors == 2 || active_neighbors == 3 {
                    new_grid[y][x] = true;
                }
            } else {
                if active_neighbors == 3 {
                    new_grid[y][x] = true;
                }
            }
        }
    }

    if corners_stay {
        let height = grid.len();
        let width = grid[0].len();

        new_grid[0][0] = true;
        new_grid[0][width - 1] = true;
        new_grid[height - 1][0] = true;
        new_grid[height - 1][width - 1] = true;
    }

    new_grid
}

fn part1(input: &str, steps: i32) -> String {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for _ in 0..steps {
        grid = grid_step(grid, false);
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&c| c).count())
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str, steps: i32) -> String {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    grid[0][0] = true;
    grid[0][width - 1] = true;
    grid[height - 1][0] = true;
    grid[height - 1][width - 1] = true;

    for _ in 0..steps {
        grid = grid_step(grid, true);
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&c| c).count())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day18_part1() {
        let input = include_str!("../../inputs/in18ex.txt");
        assert_eq!(part1(input, 4), "4");
    }

    #[test]
    fn day18_part2() {
        let input = include_str!("../../inputs/in18ex.txt");
        assert_eq!(part2(input, 5), "17");
    }
}
