use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67124473: FileFormat = FileFormat {
    id: 67_124_473,
    source_type: SourceType::Wikidata,
    name: "Print Artist letterhead file format",
    extensions: &["lth"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
