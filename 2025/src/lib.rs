pub mod solutions;

#[macro_export]
macro_rules! day_test {
    (
        $mod_name:ident,
        $day:literal,
        example = ($ex_p1:literal, $ex_p2:literal),
        input   = ($in_p1:literal, $in_p2:literal)
    ) => {
        #[cfg(test)]
        mod $mod_name {
            use super::solve;
            use anyhow::Result;
            use std::fs;

            #[test]
            fn test_example() -> Result<()> {
                let path = format!("examples/day{:02}.txt", $day);
                let input = fs::read_to_string(path)?;

                assert_eq!(
                    solve(input.trim())?,
                    ($ex_p1.to_string(), $ex_p2.to_string())
                );
                Ok(())
            }

            #[test]
            fn test_input() -> Result<()> {
                let path = format!("inputs/day{:02}.txt", $day);
                let input = fs::read_to_string(path)?;

                assert_eq!(
                    solve(input.trim())?,
                    ($in_p1.to_string(), $in_p2.to_string())
                );
                Ok(())
            }
        }
    };
}
