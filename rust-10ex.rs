use std::io::{self, Read};

// Декодер, который сдвигает буквы (например ROT13)
struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.input.read(buf)?;

        for b in buf.iter_mut().take(n) {
            if b.is_ascii_lowercase() {
                *b = (*b - b'a' + self.rot) % 26 + b'a';
            } else if b.is_ascii_uppercase() {
                *b = (*b - b'A' + self.rot) % 26 + b'A';
            }
        }

        Ok(n)
    }
}

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13,
    };

    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();

    println!("{}", result);
}