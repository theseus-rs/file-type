use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207469: FileFormat = FileFormat {
    id: 28_207_469,
    source_type: SourceType::Wikidata,
    name: "Vivid IMG",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
