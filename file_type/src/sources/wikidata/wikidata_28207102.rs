use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207102: FileFormat = FileFormat {
    id: 28_207_102,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Names file",
    extensions: &["nam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
