use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385314: FileFormat = FileFormat {
    id: 123_385_314,
    source_type: SourceType::Wikidata,
    name: "Old material library file",
    extensions: &["mlb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
