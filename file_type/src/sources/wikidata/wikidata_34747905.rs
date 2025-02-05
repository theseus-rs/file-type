use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34747905: FileFormat = FileFormat {
    id: 34_747_905,
    source_type: SourceType::Wikidata,
    name: "Swift script",
    extensions: &["swift"],
    media_types: &["text/x-swift"],
    signatures: &[],
    related_formats: &[],
};
