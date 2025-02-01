use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123385339: FileFormat = FileFormat {
    id: 123_385_339,
    puid: "wikidata/123385339",
    name: "Object library file",
    extensions: &["obl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
