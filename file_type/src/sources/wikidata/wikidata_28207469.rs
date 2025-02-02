use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207469: FileFormat = FileFormat {
    id: 28_207_469,
    source_type: SourceType::Wikidata,
    name: "Vivid IMG",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
