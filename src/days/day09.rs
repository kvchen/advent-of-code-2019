use crate::intcode::Computer;
use anyhow::Result;

pub fn part1(source: &str) -> Result<String> {
    let output = Computer::new_from_str(source)?.run(vec![1])?;
    Ok(format!("{:?}", output))
}

pub fn part2(source: &str) -> Result<String> {
    let output = Computer::new_from_str(source)?.run(vec![2])?;
    Ok(format!("{:?}", output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quine() -> Result<()> {
        let source = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        assert_eq!(Computer::new(&source).run(vec![])?, source);
        Ok(())
    }

    #[test]
    fn test_sixteen_digit() -> Result<()> {
        let source = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        assert_eq!(Computer::new(&source).run(vec![])?, vec![1219070632396864]);
        Ok(())
    }

    #[test]
    fn test_large_number() -> Result<()> {
        let source = vec![104, 1125899906842624, 99];

        assert_eq!(Computer::new(&source).run(vec![])?, vec![1125899906842624]);
        Ok(())
    }
}
