use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861035: FileFormat = FileFormat {
    id: 105_861_035,
    source_type: SourceType::Wikidata,
    name: "CWLS Log ASCII Standard (with rem)",
    extensions: &["las"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
