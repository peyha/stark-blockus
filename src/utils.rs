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