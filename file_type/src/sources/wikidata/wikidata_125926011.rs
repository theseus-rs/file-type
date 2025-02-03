use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125926011: FileFormat = FileFormat {
    id: 125_926_011,
    source_type: SourceType::Wikidata,
    name: "Final Writer Document",
    extensions: &["fw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
