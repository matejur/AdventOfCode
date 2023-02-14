if [ -z "$1" ]; then
    echo "No day number provided!"
    exit
fi

day=$(printf "%02d" $1)

file=src/solutions/day${day}.rs

if [ -f "$file" ]; then
    echo "File already exists"
    exit
fi

cat > "$file" << EOF
pub fn solve() {
    let input = include_str!("../../inputs/in${day}.txt");
    println!("Solving day $1");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> String {
    "result".to_string()
}

fn part2(input: &str) -> String {
    "result".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day${day}_part1() {

    }

    #[test]
    #[ignore]
    fn day${day}_part2() {

    }
}
EOF

echo "pub mod day$day;" >> src/solutions/mod.rs
touch inputs/in$day.txt
echo "Created everything. Add the day to main.rs match"