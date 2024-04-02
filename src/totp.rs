
use std::vec;

use super::crypt::digest;
use super::crypt::Block;

const DIGIT:usize= 6;

fn hmac(key: &[u8], message: &str) -> [u8; 20] {
    let key_bytes: Vec<u8> = if key.len() > 64 {
        [digest(Block::from_bytes(key.to_vec())).to_vec(), vec![0_u8; 64 - 20]].concat()
    } else {
        [key.to_vec(), vec![0_u8; 64 - key.len()]].concat()
    };
    assert_eq!(key_bytes.len(), 64);

    let mut ipad = vec![0x36_u8; 64];
    for (i, &byte) in key_bytes.iter().enumerate() {
        ipad[i] ^= byte;
    }
    ipad.extend_from_slice(message.as_bytes());
    

    let mut opad = vec![0x5C_u8; 64];
    for (i, &byte) in key_bytes.iter().enumerate() {
        opad[i] ^= byte;
    }

    opad.extend_from_slice(&digest(Block::from_bytes(ipad)));
    digest(Block::from_bytes(opad))
}

fn totp(key: &str, message:&str)->u32{
    //truncate
    let digest = hmac(key.as_bytes(), message);
    let offset = usize::try_from(digest[19] & 0x0F).expect("offset is in range of 0..16"); //last 4 bits
    let sum =u32::from_be_bytes([digest[offset],digest[offset+1],digest[offset+2],digest[offset+3]]) & 0x7FFFFFFF;
    sum % 1_000_000
}

#[cfg(test)]
mod tests {


    use super::hmac;

    #[test]
    fn test_hmac() {
        let key =  [0x0b_u8 ; 20];
        let digest = hmac(key.as_slice(), "Hi There");
        assert_eq!(digest, [0xb6, 0x17, 0x31, 0x86, 0x55, 0x05, 0x72, 0x64, 0xe2, 0x8b, 0xc0, 0xb6, 0xfb, 0x37,0x8c,0x8e,0xf1,0x46,0xbe,0x00]);
    }

    #[test]
    fn test_hmac2() {
        let key =  b"Jefe";
        let digest = hmac(key.as_slice(), "what do ya want for nothing?");
        assert_eq!(digest, [0xef, 0xfc, 0xdf, 0x6a, 0xe5, 0xeb, 0x2f, 0xa2, 0xd2, 0x74, 0x16, 0xd5, 0xf1, 0x84,0xdf,0x9c,0x25,0x9a,0x7c,0x79]);
    }
}