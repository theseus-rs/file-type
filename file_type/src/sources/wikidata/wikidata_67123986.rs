use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123986: FileFormat = FileFormat {
    id: 67_123_986,
    source_type: SourceType::Wikidata,
    name: "Print Artist envelope file format",
    extensions: &["env"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
