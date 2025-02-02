use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123385314: FileFormat = FileFormat {
    id: 123_385_314,
    source_type: SourceType::Wikidata,
    name: "Old material library file",
    extensions: &["mlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
