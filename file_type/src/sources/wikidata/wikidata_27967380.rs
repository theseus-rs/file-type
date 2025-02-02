use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967380: FileFormat = FileFormat {
    id: 27_967_380,
    source_type: SourceType::Wikidata,
    name: "EVT",
    extensions: &["evt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
