use std::io::BufRead;

pub struct Reader {
    lines: std::iter::Enumerate<std::io::Lines<std::io::BufReader<std::fs::File>>>,
}

impl Reader {
    pub fn new(file_name: &str) -> Self {
        let file =
            std::fs::File::open(file_name).expect(&format!("File: {file_name} could not be found"));
        let reader = std::io::BufReader::new(file);
        let lines = reader.lines().enumerate();

        Self { lines }
    }

    pub fn next(&mut self) -> Option<String> {
        Some(self.next_indexed()?.0)
    }

    pub fn next_indexed(&mut self) -> Option<(String, usize)> {
        loop {
            let (index, line) = self.lines.next()?;
            let string = line
                .expect(&format!("Line {} was not a string", index + 1))
                .trim()
                .to_string();
            if string.starts_with('#') {
                continue;
            } else {
                return Some((string, index));
            }
        }
    }
}
