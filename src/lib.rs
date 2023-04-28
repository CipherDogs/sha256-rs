//! sha256-rs
//!
//! Implementation of the SHA256 hash made using Rust

/// Initializing Variables
/// First 32 bits of fractional parts of the square roots of the first eight primes
const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;

/// Constants table
/// First 32 bits of the fractional parts of the cube roots of the first 64 primes
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA256 digest bytes
///
/// # Example
///
/// ```rust
/// let bytes = b"hello";
/// let result = sha256_rs::sha256(bytes);
/// assert_eq!(result, [44, 242, 77, 186, 95, 176, 163, 14, 38, 232, 59, 42, 197, 185, 226, 158, 27, 22, 30, 92, 31, 167, 66, 94, 115, 4, 51, 98, 147, 139, 152, 36])
/// ```
///
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut bytes = bytes.to_vec();
    let bytes_len = bytes.len() * 8;
    bytes.push(0x80);

    while (bytes.len() % 64) != 56 {
        bytes.push(0);
    }

    for i in bytes_len.to_be_bytes().iter() {
        bytes.push(*i);
    }

    let mut temp = vec![];

    for chunk in bytes.as_slice().chunks(64) {
        let mut w = [0; 64];

        for (w, d) in w.iter_mut().zip(chunk.iter().step_by(4)).take(16) {
            *w = u32::from_be_bytes(unsafe { *(d as *const u8 as *const [u8; 4]) });
        }

        for i in 16..64 {
            let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = H0;
        let mut b = H1;
        let mut c = H2;
        let mut d = H3;
        let mut e = H4;
        let mut f = H5;
        let mut g = H6;
        let mut h = H7;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        temp.push(H0.wrapping_add(a).to_be_bytes());
        temp.push(H1.wrapping_add(b).to_be_bytes());
        temp.push(H2.wrapping_add(c).to_be_bytes());
        temp.push(H3.wrapping_add(d).to_be_bytes());
        temp.push(H4.wrapping_add(e).to_be_bytes());
        temp.push(H5.wrapping_add(f).to_be_bytes());
        temp.push(H6.wrapping_add(g).to_be_bytes());
        temp.push(H7.wrapping_add(h).to_be_bytes());
    }

    let mut result = [0u8; 32];
    result.copy_from_slice(temp.concat().as_slice());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            sha256("Test".as_bytes()),
            [
                83, 46, 170, 189, 149, 116, 136, 13, 191, 118, 185, 184, 204, 0, 131, 44, 32, 166,
                236, 17, 61, 104, 34, 153, 85, 13, 122, 110, 15, 52, 94, 37
            ]
        );

        assert_eq!(
            sha256("Rust".as_bytes()),
            [
                217, 170, 137, 253, 209, 90, 213, 196, 29, 156, 18, 143, 239, 254, 158, 7, 220,
                130, 139, 131, 248, 82, 150, 247, 244, 43, 218, 80, 104, 33, 48, 14
            ]
        );

        assert_eq!(
            sha256("hello world".as_bytes()),
            [
                185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132,
                239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233
            ]
        );

        assert_eq!(
            sha256("Lorem ipsum dolor sit amet, consectetur adipiscing elit".as_bytes()),
            [
                7, 254, 77, 74, 37, 113, 130, 65, 175, 20, 90, 147, 248, 144, 235, 84, 105, 5, 46,
                37, 29, 25, 157, 23, 59, 211, 189, 80, 195, 187, 77, 162
            ]
        );
    }
}
