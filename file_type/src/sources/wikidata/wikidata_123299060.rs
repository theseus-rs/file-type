use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123299060: FileFormat = FileFormat {
    id: 123_299_060,
    puid: "wikidata/123299060",
    name: "Ancestry.com Family Tree Database",
    extensions: &["aft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
