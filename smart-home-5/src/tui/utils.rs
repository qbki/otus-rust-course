use std::io;

const MAX_STRING_LENGTH: usize = 80;

pub fn wrong_command(writer: &mut dyn io::Write) -> io::Result<()> {
    writeln!(writer, "Wrong command...")
}

pub fn get_name(writer: &mut dyn io::Write) -> io::Result<String> {
    loop {
        let input = get_input();

        match input {
            Result::Ok(name) => {
                if name.is_empty() {
                    writeln!(writer, "Error: A name should not be empty")?;
                    continue;
                }

                if name.chars().count() > MAX_STRING_LENGTH {
                    writeln!(
                        writer,
                        "Error: Number of characters should be less or equal to {}",
                        MAX_STRING_LENGTH
                    )?;
                    continue;
                }

                return Result::Ok(name);
            }
            Result::Err(_) => {
                writeln!(writer, "Error: Unknown error")?;
                continue;
            }
        }
    }
}

pub fn get_number(writer: &mut dyn io::Write) -> io::Result<f64> {
    loop {
        let input = get_input()?;

        match input.parse::<f64>() {
            Result::Ok(value) => {
                return Result::Ok(value);
            }
            Result::Err(_) => {
                writeln!(writer, "Error: Wrong number format")?;
            }
        }
    }
}

pub fn get_input() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        io::Result::Ok(_) => io::Result::Ok(input.trim().into()),
        io::Result::Err(error) => io::Result::Err(error),
    }
}
