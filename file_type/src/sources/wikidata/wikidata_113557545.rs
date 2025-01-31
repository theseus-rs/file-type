use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113557545: FileFormat = FileFormat {
    id: 113_557_545,
    puid: "wikidata/113557545",
    name: "Gear Image",
    extensions: &["p01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
