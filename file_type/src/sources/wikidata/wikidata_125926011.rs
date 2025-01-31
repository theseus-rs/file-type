use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125926011: FileFormat = FileFormat {
    id: 125_926_011,
    puid: "wikidata/125926011",
    name: "Final Writer Document",
    extensions: &["fw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
