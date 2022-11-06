//! Helpers for adding serde support to external types.
pub(crate) mod mime {
    use mime::Mime;
    use serde::{
        de::{Deserializer, Error, Visitor},
        ser::Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(
        source: &Option<Mime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(mime) = source {
            serializer.serialize_str(&mime.to_string())
        } else {
            serializer.serialize_none()
        }
    }

    struct MimeVisitor;

    impl<'de> Visitor<'de> for MimeVisitor {
        type Value = Mime;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mime: Mime = v.parse().map_err(Error::custom)?;
            Ok(mime)
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Mime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = deserializer.deserialize_str(MimeVisitor)?;
        Ok(Some(result))
    }
}
