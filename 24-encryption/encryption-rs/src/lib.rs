pub mod xor {
    #![allow(dead_code)]

    pub fn encrypt(buf: &[u8], key: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(buf.len());
        for i in 0..buf.len() {
            result.push(buf[i] ^ key[i % key.len()]);
        }
        result
    }
    pub fn decrypt(buf: &[u8], key: &[u8]) -> Vec<u8> {
        encrypt(buf, key)
    }
}

pub mod xor_cbc {
    #![allow(dead_code)]
    extern crate rand;
    use rand::RngCore;

    pub fn encrypt(buf: &[u8], key: &[u8], block_size: usize) -> Vec<u8> {
        debug_assert!(block_size > 0 && key.len() > 0 && buf.len() > 0);
        let mut result = Vec::with_capacity(buf.len() + block_size);
        // declare and fill initialisation vector
        let mut iv = vec![0; block_size];
        rand::thread_rng().fill_bytes(&mut iv);
        result.extend_from_slice(&iv);
        let mut enc_block = iv;
        for i in 0..buf.len() {
            let j = i % block_size;
            enc_block[j] = enc_block[j] ^ buf[i] ^ key[i % key.len()];
            result.push(enc_block[j]);
        }
        result
    }

    pub fn decrypt(buf: &[u8], key: &[u8], block_size: usize) -> Vec<u8> {
        debug_assert!(block_size > 0 && key.len() > 0 && buf.len() > 0);
        let mut result = Vec::with_capacity(buf.len());
        let mut prev_enc_block = buf[0..block_size].to_vec();
        let mut open_block = vec![0; block_size];
        let mut dec_block = vec![0; block_size];
        for i in block_size..buf.len() {
            let j = i % block_size;
            open_block[j] = buf[i] ^ key[(i - block_size) % key.len()];
            dec_block[j] = open_block[j] ^ prev_enc_block[j];
            prev_enc_block[j] = buf[i];
            result.push(dec_block[j]);
        }
        result
    }
}

pub mod rsa {
    #![allow(dead_code)]

    extern crate num;
    use num::BigInt;
    use num::ToPrimitive;

    pub fn encrypt(buf: &[u64], e: usize, n: u128) -> Vec<u64> {
        let mut result = Vec::with_capacity(buf.len());
        for b in buf {
            let x = BigInt::from(*b);
            let powered = num::pow::pow(x, e);
            let r = powered % BigInt::from(n);
            result.push(r.to_u64().unwrap());
        }
        result
    }

    pub fn decrypt(buf: &[u64], d: usize, n: u128) -> Vec<u64> {
        encrypt(buf, d, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::rsa;
    use crate::xor;
    use crate::xor_cbc;

    #[test]
    fn it_works_xor() {
        let text = "hello".as_bytes();
        let key = "xyz".as_bytes();
        let encrypted = xor::encrypt(text, key);
        let decrypted = xor::decrypt(&encrypted, key);
        assert!(text == decrypted.as_slice());
    }

    #[test]
    fn it_works_xor_cbc() {
        let text = "hello".as_bytes();
        let key = "x".as_bytes();
        let encrypted = xor_cbc::encrypt(text, key, 1);
        let decrypted = xor_cbc::decrypt(&encrypted, key, 1);
        assert!(text == decrypted.as_slice());
    }

    #[test]
    fn it_works_xor_cbc_2() {
        let text = "hello world!".as_bytes();
        let key = "42".as_bytes();
        let encrypted = xor_cbc::encrypt(text, key, 2);
        let decrypted = xor_cbc::decrypt(&encrypted, key, 2);
        assert!(text == decrypted.as_slice());
    }

    #[test]
    fn it_works_xor_cbc_3() {
        let text = "hello world!".as_bytes();
        let key = "42".as_bytes();
        let encrypted = xor_cbc::encrypt(text, key, 4);
        let decrypted = xor_cbc::decrypt(&encrypted, key, 4);
        assert!(text == decrypted.as_slice());
    }

    #[test]
    fn it_works_xor_cbc_4() {
        let text = "hello".as_bytes();
        let key = "kitty!!".as_bytes();
        let encrypted = xor_cbc::encrypt(text, key, 3);
        let decrypted = xor_cbc::decrypt(&encrypted, key, 3);
        assert!(text == decrypted.as_slice());
    }

    #[test]
    fn it_works_rsa() {
        let text: Vec<u64> = "hello world!"
            .as_bytes()
            .iter()
            .map(|x| *x as u64)
            .collect();
        let e = 17;
        let d = 413;
        let n = 3233;

        let encrypted = rsa::encrypt(&text, e, n);
        let decrypted = rsa::decrypt(&encrypted, d, n);
        assert!(text == decrypted.as_slice());
    }
}
