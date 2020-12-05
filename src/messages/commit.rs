use crate::errors::*;
use bytes::*;
use std::convert::TryInto;

pub const MARKER: u8 = 0xB0;
pub const SIGNATURE: u8 = 0x12;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Commit;

impl Commit {
    pub fn new() -> Commit {
        Commit {}
    }
}

impl TryInto<Bytes> for Commit {
    type Error = Error;
    fn try_into(self) -> Result<Bytes> {
        Ok(Bytes::from_static(&[MARKER, SIGNATURE]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_serialize_commit() {
        let commit = Commit::new();

        let bytes: Bytes = commit.try_into().unwrap();

        assert_eq!(bytes, Bytes::from_static(&[MARKER, SIGNATURE,]));
    }
}
