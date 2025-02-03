use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123385496: FileFormat = FileFormat {
    id: 123_385_496,
    source_type: SourceType::Wikidata,
    name: "Path library file",
    extensions: &["ptl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
