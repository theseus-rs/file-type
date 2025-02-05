use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861035: FileFormat = FileFormat {
    id: 105_861_035,
    source_type: SourceType::Wikidata,
    name: "CWLS Log ASCII Standard (with rem)",
    extensions: &["las"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
