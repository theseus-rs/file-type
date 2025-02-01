use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123456229: FileFormat = FileFormat {
    id: 123_456_229,
    puid: "wikidata/123456229",
    name: "CFW Form file",
    extensions: &["cfw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
