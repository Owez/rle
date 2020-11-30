//! A simple [run-length-encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
//! implementation for light compression
//!
//! ## Examples
//!
//! Encoding:
//!
//! ```rust
//! fn main() {
//!     let data = &[44, 43, 6, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 3];
//!     let compressed = rle::compress(data);
//!
//!     println!("normal: {}, compressed: {}", data.len(), compressed.len());
//!
//!     // will show "normal: 16, compressed: 10"
//! }
//! ```
//!
//! ## Under the hood
//!
//! This `rle` library uses a custom [run-length-encoding](https://en.wikipedia.org/wiki/Run-length_encoding)
//! technique in order to get the most efficiant compression results. It achieves
//! this by allowing all repeating characters under 6x through, e.g:
//!
//! ```none
//! helllllo!
//! ```
//!
//! But allows anything 6x or above through, e.g:
//!
//! ```none
//! hellllllllllllllllllllllllllllllllllllllllllllo!
//! ```
//!
//! This is due to the encoding using a `u32` under the hood to store the length,
//! which means the it can store up to ~4 billion repeating characters until
//! overflow. A run-length-encoded block would look like the following for the
//! previous example:
//!
//! ```none
//! [h, e, 4, 0, 0, 0, 44, o, !]
//! ```
//!
//! You may assume whatever binary encoding you'd like for these letters to
//! properly expand this block, but in essense it uses an [End-of-Transmission character](https://en.wikipedia.org/wiki/End-of-Transmission_character)
//! to represent the start of an run-length-encoded block and has a `[u8; 4]`
//! (which represents the previously mentioned `u32` in big-endian form).
//!
//! After that, it simply has a `u8` for the byte it is representing and continued
//! further onwards; looping this compression/decompression until the end of the
//! inputted bytes.

/// The [End-of-Transmission character](https://en.wikipedia.org/wiki/End-of-Transmission_character),
/// which in ASCII and Unicode is the 4th character
const END_OF_TRANSMISSION: u8 = 4;

/// Compresses to custom `rle` from given bytes
///
/// # Example
///
/// ```rust
/// fn main() {
///     let data = &[44, 43, 6, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 3];
///     let compressed = rle::compress(data);
///
///     println!("normal: {}, compressed: {}", data.len(), compressed.len());
///
///     // will show "normal: 16, compressed: 10"
/// }
/// ```
pub fn compress(data: impl AsRef<[u8]>) -> Vec<u8> {
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

// TODO: decoding

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_change_compress() {
        let exp1 = &[0, 1, 2, 3, 4, 5, 6, 7];
        let exp2 = &[0, 0, 0, 0, 0];
        let exp3 = &[0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];

        assert_eq!(compress(exp1), exp1);
        assert_eq!(compress(exp2), exp2);
        assert_eq!(compress(exp3), exp3);
    }

    #[test]
    fn simple_compress() {
        let six = 6u32.to_be_bytes();
        let sixty_four = 64u32.to_be_bytes();

        assert_eq!(
            compress(&[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1]),
            &[
                END_OF_TRANSMISSION,
                six[0],
                six[1],
                six[2],
                six[3],
                0,
                END_OF_TRANSMISSION,
                six[0],
                six[1],
                six[2],
                six[3],
                1
            ]
        );

        assert_eq!(
            compress(&[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 64, 64, 230
            ]),
            &[
                END_OF_TRANSMISSION,
                sixty_four[0],
                sixty_four[1],
                sixty_four[2],
                sixty_four[3],
                0,
                64,
                64,
                230
            ]
        );
    }
}
