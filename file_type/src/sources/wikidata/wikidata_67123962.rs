use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123962: FileFormat = FileFormat {
    id: 67_123_962,
    source_type: SourceType::Wikidata,
    name: "Print Artist calendar file format",
    extensions: &["cal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
