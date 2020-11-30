# Run-length-encoding: `rle`

A simple [run-length-encoding](https://en.wikipedia.org/wiki/Run-length_encoding) implementation for light compression

## Examples

Encoding:

```rust
use rle;

fn main() {
    let data = &[44, 43, 6, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 3];
    let compressed = rle::compress(data);

    println!("normal: {}, compressed: {}", data.len(), compressed.len());

    // will show "normal: 16, compressed: 10"
}
```

## Under the hood

This `rle` library uses a custom [run-length-encoding](https://en.wikipedia.org/wiki/Run-length_encoding) technique in order to get the most efficiant compression results. It achieves this by allowing all repeating characters under 6x through, e.g:

```none
helllllo!
```

But allows anything 6x or above through, e.g:

```none
hellllllllllllllllllllllllllllllllllllllllllllo!
```

This is due to the encoding using a `u32` under the hood to store the length, which means the it can store up to ~4 billion repeating characters until overflow. A run-length-encoded block would look like the following for the previous example:

```none
[h, e, 4, 0, 0, 0, 44, o, !]
```

You may assume whatever binary encoding you'd like for these letters to properly expand this block, but in essense it uses an [End-of-Transmission character](https://en.wikipedia.org/wiki/End-of-Transmission_character) to represent the start of an run-length-encoded block and has a `[u8; 4]` (which represents the previously mentioned `u32` in big-endian form).

After that, it simply has a `u8` for the byte it is representing and continued further onwards; looping this compression/decompression until the end of the inputted bytes.
