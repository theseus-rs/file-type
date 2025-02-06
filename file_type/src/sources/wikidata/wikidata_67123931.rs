use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123931: FileFormat = FileFormat {
    id: 67_123_931,
    source_type: SourceType::Wikidata,
    name: "Print Artist banner file format",
    extensions: &["ban"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
