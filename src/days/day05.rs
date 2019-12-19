use crate::intcode::Computer;
use anyhow::Result;

pub fn part1(source: &str) -> Result<String> {
    let output = Computer::new_from_str(source)?.run(vec![1])?;
    Ok(format!("{:?}", output))
}

pub fn part2(source: &str) -> Result<String> {
    let output = Computer::new_from_str(source)?.run(vec![5])?;
    Ok(format!("{:?}", output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_equals() -> Result<()> {
        let source = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(Computer::new(&source).run(vec![8])?, vec![1]);
        assert_eq!(Computer::new(&source).run(vec![2])?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_position_less_than() -> Result<()> {
        let source = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(Computer::new(&source).run(vec![7])?, vec![1]);
        assert_eq!(Computer::new(&source).run(vec![9])?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_immediate_equals() -> Result<()> {
        let source = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(Computer::new(&source).run(vec![8])?, vec![1]);
        assert_eq!(Computer::new(&source).run(vec![2])?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_immediate_less_than() -> Result<()> {
        let source = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(Computer::new(&source).run(vec![7])?, vec![1]);
        assert_eq!(Computer::new(&source).run(vec![9])?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_position_jump() -> Result<()> {
        let source = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(Computer::new(&source).run(vec![0])?, vec![0]);
        assert_eq!(Computer::new(&source).run(vec![42])?, vec![1]);

        Ok(())
    }

    #[test]
    fn test_immediate_jump() -> Result<()> {
        let source = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        assert_eq!(Computer::new(&source).run(vec![0])?, vec![0]);
        assert_eq!(Computer::new(&source).run(vec![42])?, vec![1]);

        Ok(())
    }

    #[test]
    fn test_large_example() -> Result<()> {
        let source = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        assert_eq!(Computer::new(&source).run(vec![7])?, vec![999]);
        assert_eq!(Computer::new(&source).run(vec![8])?, vec![1000]);
        assert_eq!(Computer::new(&source).run(vec![9])?, vec![1001]);

        Ok(())
    }
}
