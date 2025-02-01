use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123594002: FileFormat = FileFormat {
    id: 123_594_002,
    puid: "wikidata/123594002",
    name: "Norton Change Directory Persistent Cache File",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
