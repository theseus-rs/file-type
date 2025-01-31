use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123693374: FileFormat = FileFormat {
    id: 123_693_374,
    puid: "wikidata/123693374",
    name: "Pascal unit file",
    extensions: &["pas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
