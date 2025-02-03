use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67123986: FileFormat = FileFormat {
    id: 67_123_986,
    source_type: SourceType::Wikidata,
    name: "Print Artist envelope file format",
    extensions: &["env"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
