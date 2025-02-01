use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385294: FileFormat = FileFormat {
    id: 123_385_294,
    puid: "wikidata/123385294",
    name: "Material library file",
    extensions: &["mtl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
