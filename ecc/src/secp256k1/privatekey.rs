use crate::secp256k1::{
    constants::{G, N},
    s256point::S256Point,
    signature::Signature,
};
use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub secret: BigUint,
    pub point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: BigUint) -> Self {
        let point = G.clone() * secret.clone();
        Self { secret, point }
    }

    pub fn hex(&self) -> String {
        format!("{:064x}", self.secret)
    }

    pub fn sign(&self, z: BigUint) -> Signature {
        let mut rng = thread_rng();
        let k = rng.gen_biguint_below(&N);
        let r_point = G.clone() * k.clone();
        let r = r_point.inner().x.as_ref().unwrap().num.clone();

        let k_inv = k.modpow(&(N.clone() - BigUint::from(2u8)), &N);
        let mut s = ((z + &r * &self.secret) * k_inv) % N.clone();

        // enforce low-S values
        if s > N.clone() / BigUint::from(2u8) {
            s = N.clone() - s;
        }

        Signature { r, s }
    }
}

#[cfg(test)]
mod tests {
    use crate::secp256k1::{
        constants::{G, N},
        privatekey::PrivateKey,
        s256point::S256Point,
    };
    use num_bigint::BigUint;
    use sha2::{Digest, Sha256};

    fn hash256(msg: &[u8]) -> Vec<u8> {
        let first = Sha256::digest(msg);
        Sha256::digest(&first).to_vec()
    }

    #[test]
    fn test_sign_programming_bitcoin_message() {
        let e = BigUint::from(12345u32);
        let z = BigUint::from_bytes_be(&hash256(b"Programming Bitcoin!"));

        let k = BigUint::from(1234567890u64);
        let r_point = G.clone() * k.clone();
        let r = r_point.inner().x.as_ref().unwrap().num.clone();
        let k_inv = k.modpow(&(N.clone() - BigUint::from(2u8)), &N);
        let s = ((&z + &r * &e) * k_inv) % N.clone();

        let expected_r = BigUint::parse_bytes(
            b"2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22",
            16,
        )
        .unwrap();

        let expected_s = BigUint::parse_bytes(
            b"1dbc63bfef4416705e602a7b564161167076d8b20990a0f26f316cff2cb0bc1a",
            16,
        )
        .unwrap();

        let expected_point = S256Point::new(
            Some(
                BigUint::parse_bytes(
                    b"f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f",
                    16,
                )
                .unwrap(),
            ),
            Some(
                BigUint::parse_bytes(
                    b"0eba29d0f0c5408ed681984dc525982abefccd9f7ff01dd26da4999cf3f6a295",
                    16,
                )
                .unwrap(),
            ),
        );

        let privkey = PrivateKey::new(e.clone());

        assert_eq!(privkey.point, expected_point);
        assert_eq!(r, expected_r);
        assert_eq!(s, expected_s);
    }
}
