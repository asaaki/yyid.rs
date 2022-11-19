//! Conversions between UUIDs and YYIDs

use super::Yyid;
use uuid::Uuid;

impl From<Uuid> for Yyid {
    #[inline]
    fn from(f: Uuid) -> Self {
        // SAFETY: `Uuid` and `Yyid` have the same ABI;
        //         they're both transparent wrappers around `[u8; 16]`
        unsafe { std::mem::transmute::<Uuid, Yyid>(f) }
    }
}

impl From<&Uuid> for Yyid {
    #[inline]
    fn from(f: &Uuid) -> Self {
        Yyid(f.into_bytes())
    }
}

// TODO: TryFrom's for YYID->UUID (fallible, since not all YYIDs are also valid UUIDs)

#[cfg(test)]
mod tests {
    use crate::{std::string::ToString, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_yyid_from_uuid() {
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = uuid.as_hyphenated().to_string();
        let yyid = Yyid::from(uuid);
        let yyid_s = yyid.as_hyphenated().to_string();

        assert_eq!(uuid_s, yyid_s);
    }

    #[test]
    fn test_uuid_into_yyid() {
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = uuid.as_hyphenated().to_string();
        let yyid: Yyid = uuid.into();
        let yyid_s = yyid.as_hyphenated().to_string();

        assert_eq!(uuid_s, yyid_s);
    }

    #[test]
    fn test_yyid_from_uuid_ref() {
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = uuid.as_hyphenated().to_string();
        let yyid = Yyid::from(&uuid);
        let yyid_s = yyid.as_hyphenated().to_string();

        assert_eq!(uuid_s, yyid_s);
    }

    #[test]
    fn test_uuid_ref_into_yyid() {
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = uuid.as_hyphenated().to_string();
        let uuid_ref = &uuid;
        let yyid: Yyid = uuid_ref.into();
        let yyid_s = yyid.as_hyphenated().to_string();

        assert_eq!(uuid_s, yyid_s);
    }
}
