use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123436632: FileFormat = FileFormat {
    id: 123_436_632,
    source_type: SourceType::Wikidata,
    name: "Construction File",
    extensions: &["cf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
