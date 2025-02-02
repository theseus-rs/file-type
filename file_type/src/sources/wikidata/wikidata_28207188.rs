use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207188: FileFormat = FileFormat {
    id: 28_207_188,
    source_type: SourceType::Wikidata,
    name: "QDV",
    extensions: &["qdv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
