use std::str;

pub fn solve() {
    let input = include_str!("../../inputs/in11.txt");
    println!("Solving day 11");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn increment(passwd: &mut Vec<u8>) {
    let mut done = false;
    for char in passwd.iter_mut().rev() {
        if !done && *char < 'z' as u8 {
            *char += 1;
            done = true;
        } else if !done {
            *char = 'a' as u8;
        } else {
            break;
        }
    }
}

fn blacklisted(passwd: &mut Vec<u8>) {
    let mut found = false;
    for char in passwd.iter_mut() {
        if found {
            *char = 'a' as u8;
            continue;
        }

        match *char as char {
            'i' | 'l' | 'o' => {
                *char += 1;
                found = true;
            },
            _ => ()
        }
    }
}

fn increasing(passwd: &Vec<u8>) -> bool {
    let mut len = 0;
    
    for pair in passwd.windows(2) {
        if pair[0] + 1 == pair[1] {
            len += 1;
        } else {
            len = 0;
        }

        if len > 1 {
            return true;
        }
    }
    
    false
}

fn two_pairs(passwd: &Vec<u8>) -> bool {
    let mut found_one = false;
    let mut skip = false;

    for i in 0..passwd.len() - 1 {
        if skip {
            skip = false;
            continue;
        }

        if passwd[i] == passwd[i + 1] {
            if found_one {
                return true;
            }

            found_one = true;
            skip = true;
        }
    }

    false
}

fn next_passwd(passwd: &mut Vec<u8>) {
    while !(two_pairs(&passwd) && increasing(&passwd)) {
        increment(passwd);
        blacklisted(passwd);
    }
}

fn part1(input: &str) -> String {
    let mut passwd = input.as_bytes().to_owned();

    next_passwd(&mut passwd);

    String::from_utf8(passwd).unwrap()
}

fn part2(input: &str) -> String {
    let mut passwd = input.as_bytes().to_owned();

    next_passwd(&mut passwd);
    increment(&mut passwd);
    next_passwd(&mut passwd);

    String::from_utf8(passwd).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{two_pairs, next_passwd, increasing};

    #[test]
    fn day11_part1() {
        let in1 = "hijklmmn".as_bytes().to_owned();
        assert_eq!(increasing(&in1), true);

        let in2 = "abbceffg".as_bytes().to_owned();
        assert_eq!(two_pairs(&in2), true);
        assert_eq!(increasing(&in2), false);

        let in3 = "abbcegjk".as_bytes().to_owned();
        assert_eq!(two_pairs(&in3), false);

        let mut in4 = "abcdefgh".as_bytes().to_owned();
        next_passwd(&mut in4);
        assert_eq!(String::from_utf8(in4).unwrap(), "abcdffaa");

        let mut in5 = "ghijklmn".as_bytes().to_owned();
        next_passwd(&mut in5);
        assert_eq!(String::from_utf8(in5).unwrap(), "ghjaabcc");
    }
}
