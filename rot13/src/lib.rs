use std::io::BufWriter;
use std::io::Write;

pub struct Rot13Writer<T>
where
    T: Write,
{
    writer: BufWriter<T>
}

impl<T> Rot13Writer<T>
where
    T: Write,
{
    pub fn new(inner: T) -> Self {
        Rot13Writer { writer: BufWriter::new(inner) }
    }
}

impl<T> Write for Rot13Writer<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Write byte by byte from buffer to writer
		for (i, ch) in buf.iter().enumerate() {
			let rotated = rot13(*ch as char);
            match self.writer.write(&[rotated]) {
                Err(_) => return Ok(i), // On error, return number of bytes written
                Ok(_) => continue,
            }
		}

        // If we are here, all buffer was written
		Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

pub fn rot13(c: char) -> u8 {
    match c {
        'A'..='M' | 'a'..='m' => (c as u8) + 13,
        'N'..='Z' | 'n'..='z' => (c as u8) - 13,
        _ => c as u8
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::io::Write;
    use super::{Rot13Writer, rot13};

    #[test]
    fn test_rot13_single_chars() {
        let pairs: Vec<(char, char)> = vec![
            // lower case letters
            ('a', 'n'),
            ('z', 'm'),
            // upper case letter
            ('A', 'N'),
            ('Z', 'M'),
            // special chars
            (' ', ' '),
            (';', ';'),
        ];
        for pair in pairs { 
            assert_eq!(rot13(pair.0), pair.1 as u8);
            assert_eq!(rot13(pair.1), pair.0 as u8);
        }
    }

    #[test]
    fn test_rot13_sentences() {
        assert_rot13(&String::from(""));
        assert_rot13(&String::from("Laiout FTW!"));
        assert_rot13(&String::from("Marcin Seremak"));
        assert_rot13(&String::from("Rust do it!"));
    }

    // Helper function that rotates @text twice
    fn assert_rot13(text: &String) {
        let mut content = Vec::<u8>::default();

        // Rotate input text
        {
            let mut buff = Rot13Writer::new(&mut content);
            buff.write_all(text.as_bytes()).expect("Failed to write");
        }

        // Rotate it back 
        let mut content_back = Vec::<u8>::default();
        {
            let mut buff = Rot13Writer::new(&mut content_back);
            buff.write_all(&content).expect("Failed to write");
        }

        // Convert Vec<u8> into string
        let out = content_back.iter().map(|x| *x as char).collect::<String>();

        // Make sure that rotated and rotated back text equals text
        assert_eq!(&out, text);
    }
}