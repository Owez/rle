//! A simple [run-length-encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
//! implementation for minimal compression
//!
//! # Breaking changes
//!
//! This library will not have breaking changes but is kept in a `0.0.x` state
//! in case another developer has a better use for the `rle` crate name. If you
//! do, you may contact me using the following methods:
//!
//! - [Website](https://ogriffiths.com)
//! - [Github](https://github.com/owez/)

/// The [End-of-Transmission character](https://en.wikipedia.org/wiki/End-of-Transmission_character),
/// which in ASCII and Unicode is the 4th character
const END_OF_TRANSMISSION: u8 = 4;

/// Encodes to custom `rle`, only shortens 6 repetitions or more otherwise it's
/// less efficiant to do so
pub fn encode(data: impl AsRef<[u8]>) -> Vec<u8> {
    fn compute_buf(buf: &mut (u8, u32), output: &mut Vec<u8>) {
        if buf.1 >= 6 {
            // do RLE if more efficiant to do so
            output.push(END_OF_TRANSMISSION);
            output.extend_from_slice(&buf.1.to_be_bytes());
            output.push(buf.0)
        } else {
            // add normal manual
            for _ in 0..buf.1 {
                output.push(buf.0)
            }
        }
    }

    let mut output = Vec::new();
    let mut buf: (u8, u32) = (0, 0);

    for byte in data.as_ref() {
        if *byte == buf.0 {
            buf.1 += 1;
        } else {
            compute_buf(&mut buf, &mut output);
            buf = (*byte, 1);
        }
    }

    compute_buf(&mut buf, &mut output);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_change_encode() {
        let exp1 = &[0, 1, 2, 3, 4, 5, 6, 7];
        let exp2 = &[0, 0, 0, 0, 0];
        let exp3 = &[0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];

        assert_eq!(encode(exp1), exp1);
        assert_eq!(encode(exp2), exp2);
        assert_eq!(encode(exp3), exp3);
    }
}
