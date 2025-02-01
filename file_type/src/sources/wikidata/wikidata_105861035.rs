use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861035: FileFormat = FileFormat {
    id: 105_861_035,
    puid: "wikidata/105861035",
    name: "CWLS Log ASCII Standard (with rem)",
    extensions: &["las"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
