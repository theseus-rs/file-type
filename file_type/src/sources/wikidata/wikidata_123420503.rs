use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123420503: FileFormat = FileFormat {
    id: 123_420_503,
    puid: "wikidata/123420503",
    name: "DropBox file",
    extensions: &["dbox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
