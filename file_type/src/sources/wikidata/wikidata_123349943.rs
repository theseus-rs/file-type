use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123349943: FileFormat = FileFormat {
    id: 123_349_943,
    source_type: SourceType::Wikidata,
    name: "Family Origins Database",
    extensions: &["fow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
