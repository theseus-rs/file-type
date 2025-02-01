use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123693494: FileFormat = FileFormat {
    id: 123_693_494,
    puid: "wikidata/123693494",
    name: "Module Definition file",
    extensions: &["def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
