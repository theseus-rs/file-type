use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113846496: FileFormat = FileFormat {
    id: 113_846_496,
    source_type: SourceType::Wikidata,
    name: "SureThing Template",
    extensions: &["stt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
