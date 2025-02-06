use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123981: FileFormat = FileFormat {
    id: 67_123_981,
    source_type: SourceType::Wikidata,
    name: "Print Artist craft file format",
    extensions: &["crf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
