// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::hash::{Hash, Hasher};
use sip128::{Hasher128, SipHasher13, SipHasher24};

// Hash just the bytes of the slice, without length prefix
struct Bytes<'a>(&'a [u8]);

impl<'a> Hash for Bytes<'a> {
    #[allow(unused_must_use)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Bytes(v) = *self;
        state.write(v);
    }
}

macro_rules! u8to64_le {
    ($buf:expr, $i:expr) =>
    ($buf[0+$i] as u64 |
     ($buf[1+$i] as u64) << 8 |
     ($buf[2+$i] as u64) << 16 |
     ($buf[3+$i] as u64) << 24 |
     ($buf[4+$i] as u64) << 32 |
     ($buf[5+$i] as u64) << 40 |
     ($buf[6+$i] as u64) << 48 |
     ($buf[7+$i] as u64) << 56);
    ($buf:expr, $i:expr, $len:expr) =>
    ({
        let mut t = 0;
        let mut out = 0;
        while t < $len {
            out |= ($buf[t+$i] as u64) << t*8;
            t += 1;
        }
        out
    });
}

fn hash_with<H: Hasher + Hasher128, T: Hash>(mut st: H, x: &T) -> [u8; 16] {
    x.hash(&mut st);
    st.finish128().as_bytes()
}

#[test]
#[allow(unused_must_use)]
fn test_siphash128_1_3() {
    let vecs: [[u8; 16]; 1] = [
        [
            231, 126, 188, 178, 39, 136, 165, 190, 253, 98, 219, 106, 221, 48, 48, 1
        ],
    ];

    let k0 = 0x_07_06_05_04_03_02_01_00;
    let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08;
    let mut buf = Vec::new();
    let mut out_buf = [0u8; 16];
    let mut t = 0;
    let mut state_inc = SipHasher13::new_with_keys(k0, k1);

    while t < 1 {
        let vec = vecs[t];
        let out = hash_with(SipHasher13::new_with_keys(k0, k1), &Bytes(&buf));
        assert_eq!(vec, out[..]);

        let full = hash_with(SipHasher13::new_with_keys(k0, k1), &Bytes(&buf));
        let i = state_inc.finish128().as_bytes();
        state_inc.finish_buf(&mut out_buf);

        assert_eq!(full, i);
        assert_eq!(full, out_buf[..]);
        assert_eq!(full, vec);

        buf.push(t as u8);
        Hasher::write(&mut state_inc, &[t as u8]);

        t += 1;
    }
}

#[test]
#[allow(unused_must_use)]
fn test_siphash128_2_4() {
    let vecs: [[u8; 16]; 1] = [
        [
            163, 129, 127, 4, 186, 37, 168, 230, 109, 246, 114, 20, 199, 85, 2, 147
        ],
    ];

    let k0 = 0x_07_06_05_04_03_02_01_00;
    let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08;
    let mut buf = Vec::new();
    let mut out_buf = [0u8; 32];    // deliberately use longer output buffer
    let mut t = 0;
    let mut state_inc = SipHasher24::new_with_keys(k0, k1);

    while t < 1 {
        let vec = vecs[t];
        let out = hash_with(SipHasher24::new_with_keys(k0, k1), &Bytes(&buf));
        assert_eq!(vec, out[..]);

        let full = hash_with(SipHasher24::new_with_keys(k0, k1), &Bytes(&buf));
        let i = state_inc.finish128().as_bytes();
        state_inc.finish_buf(&mut out_buf);

        assert_eq!(full, i);
        assert_eq!(full, out_buf[0..16]);
        assert!(full != out_buf[16..32]);
        assert_eq!(full, vec);

        buf.push(t as u8);
        Hasher::write(&mut state_inc, &[t as u8]);

        t += 1;
    }
}
