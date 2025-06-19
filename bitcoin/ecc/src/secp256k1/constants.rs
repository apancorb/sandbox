use crate::secp256k1::s256point::S256Point;
use lazy_static::lazy_static;
use num_bigint::BigUint;

lazy_static! {
    pub static ref P: BigUint = {
        let two = BigUint::from(2u8);
        two.pow(256) - two.pow(32) - BigUint::from(977u32)
    };
    pub static ref N: BigUint = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16
    )
    .unwrap();
    pub static ref GX: BigUint = BigUint::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16
    )
    .unwrap();
    pub static ref GY: BigUint = BigUint::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16
    )
    .unwrap();
    pub static ref G: S256Point = S256Point::new(Some(GX.clone()), Some(GY.clone()));
}
