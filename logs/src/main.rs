use std::{ fs, io::Error };

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = extract_errors(&text);
    fs::write("errors.txt", errors.join("\n"))?;

    Ok(())
}
