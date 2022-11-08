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
            serializer.serialize_str(mime.as_ref())
        } else {
            serializer.serialize_none()
        }
    }

    struct MimeVisitor;

    impl<'de> Visitor<'de> for MimeVisitor {
        type Value = Option<Mime>;

        fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let mime: Mime = v.parse().map_err(Error::custom)?;
            Ok(Some(mime))
        }

        fn visit_some<D>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(self)
        }

        // NOTE: we don't need to implement visit_none()
        // NOTE: as we use skip_serializing_if on these properties
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Mime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(MimeVisitor)
    }
}
