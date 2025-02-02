use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113846496: FileFormat = FileFormat {
    id: 113_846_496,
    source_type: SourceType::Wikidata,
    name: "SureThing Template",
    extensions: &["stt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
