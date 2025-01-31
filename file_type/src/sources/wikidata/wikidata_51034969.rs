use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51034969: FileFormat = FileFormat {
    id: 51_034_969,
    puid: "wikidata/51034969",
    name: "Paradox Database Table format, version 3",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
