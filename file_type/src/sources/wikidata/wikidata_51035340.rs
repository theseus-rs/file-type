use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51035340: FileFormat = FileFormat {
    id: 51_035_340,
    puid: "wikidata/51035340",
    name: "Paradox Database Table, version 5",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
