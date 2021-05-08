use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UUID {
    bytes: [u8; 16],
}

impl UUID {
    pub fn with(bytes: [u8; 16]) -> UUID {
        UUID { bytes: bytes }
    }
}

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This type does not accept bad values.
        let u = uuid::Uuid::from_slice(&self.bytes).unwrap();
        write!(f, "{}", &u.to_string())
    }
}


// pub fn new_v4() -> UUID {

// }

