use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123680176: FileFormat = FileFormat {
    id: 123_680_176,
    puid: "wikidata/123680176",
    name: "S-57 Electronic Navigational Chart 3.1",
    extensions: &["000", "001", "002", "003", "004", "005", "006"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
