use license_key::{self, ByteCheck, HexFormat, KeyHasher, LicenseKey, Status, Verifier};
struct Hash {}
impl KeyHasher for Hash {
    fn hash(&self, seed: u64, a: u64, b: u64, c: u64) -> u8 {
        ((seed ^ a ^ b ^ c) & 0xFF) as u8
    }
}

pub struct KeyVerifer {}

impl KeyVerifer {
    pub fn verify(key: &str) -> std::result::Result<Status, Status> {
        let verifier = Verifier::new(Hash {}, vec![ByteCheck::new(0, (79, 238, 57))]);

        let key = LicenseKey::parse::<HexFormat>(key);
        println!("{:?}", key);
        if key.get_bytes().len() == 0 {
            Err(Status::Invalid)
        } else {
            match verifier.verify(&key) {
                Status::Valid => Ok(Status::Valid),
                Status::Invalid => Err(Status::Invalid),
                Status::Blocked => Err(Status::Blocked),
                Status::Forged => Err(Status::Forged),
            }
        }
    }
}
