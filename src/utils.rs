use serde_json::Value;
use anyhow::{anyhow, Result};
// Parse a serde Value which should represent a hexadecimal value to a int
pub fn parse_hexa_value(input: &Value) -> Result<u64> {
    Ok(u64::from_str_radix(
        input
            .as_str()
            .ok_or(anyhow!("hexa fail"))?
            .trim_start_matches("0x"),
        16,
    )?)
}


pub enum DisplayType {
    SingleLine,
    DoubleLine,
}

pub fn display_pretty_block(lines: Vec<String>, display: DisplayType) -> Result<()> {

    let mut l: usize = 0;

    for line in lines.iter() {
        l = usize::max(line.len(), l);
    }
    
    let first_line = match display {
        DisplayType::DoubleLine => format!("╔{}╗", "═".repeat(l)),
        DisplayType::SingleLine => format!("┌{}┐", "─".repeat(l)),
    };

    println!("{}", first_line);
    for line in lines {
        let line = match display {
            DisplayType::DoubleLine => format!("║{}{}║", line, " ".repeat(l-line.len())),
            DisplayType::SingleLine => format!("│{}{}│", line, " ".repeat(l-line.len())),
        };
        println!("{}", line);
    }
    let last_line = match display {
        DisplayType::DoubleLine => format!("╚{}╝", "═".repeat(l)),
        DisplayType::SingleLine => format!("└{}┘", "─".repeat(l)),
    };
    println!("{}", last_line);

    Ok(())
}