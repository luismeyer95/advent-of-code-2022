use std::error::Error;

pub fn parse_lines() -> Result<Vec<String>, Box<dyn Error>> {
    std::io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}

pub fn parse_line() -> Result<String, Box<dyn Error>> {
    Ok(parse_lines()?.join("\n"))
}
