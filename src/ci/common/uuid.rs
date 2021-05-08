use std::fmt;
use crate::common::CIResult;
use uuid::Uuid as RawUUID;



/////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
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
        let u = RawUUID::from_slice(&self.bytes).unwrap();
        write!(f, "{}", &u.to_string())
    }
}









/////////////////////////////////////////////////////////////////////////////////////////////////////////

impl std::convert::TryFrom<&str> for UUID {
    type Error = Box<dyn std::error::Error>;
    fn try_from(s: &str) -> CIResult<UUID> {
        let u = uuid::Uuid::parse_str(s)?;
        let x = UUID {
            bytes: *u.as_bytes(),
        };
        Ok(x)
    }
}
impl std::convert::From<&UUID> for String {
    fn from(u: &UUID) -> String {
        format!("{}", u)
    }
}






/////////////////////////////////////////////////////////////////////////////////////////////////////////

impl serde::Serialize for UUID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let ru = RawUUID::from_bytes(self.bytes);
        let s = ru.to_string();
        serializer.serialize_str(&s)
    }
}
impl<'de> serde::Deserialize<'de> for UUID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {        
        let s = String::deserialize(deserializer)?;
        let ru = match RawUUID::parse_str(&s) {
            Err(err) => return Err(serde::de::Error::custom(err)),
            Ok(x) => x,
        };
        let u = UUID { bytes: *ru.as_bytes() };
        Ok(u)
    }
}
