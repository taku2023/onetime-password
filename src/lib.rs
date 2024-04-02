mod totp;
mod crypt;

pub fn gen_totp(key: &[u8]) -> u32 {
    totp::totp(key)
}