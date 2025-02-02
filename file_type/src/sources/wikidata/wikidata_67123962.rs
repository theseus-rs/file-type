use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67123962: FileFormat = FileFormat {
    id: 67_123_962,
    source_type: SourceType::Wikidata,
    name: "Print Artist calendar file format",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
