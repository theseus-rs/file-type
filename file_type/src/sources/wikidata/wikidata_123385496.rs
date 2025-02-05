use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385496: FileFormat = FileFormat {
    id: 123_385_496,
    source_type: SourceType::Wikidata,
    name: "Path library file",
    extensions: &["ptl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
