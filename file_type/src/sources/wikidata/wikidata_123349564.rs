use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123349564: FileFormat = FileFormat {
    id: 123_349_564,
    source_type: SourceType::Wikidata,
    name: "Clooz database file",
    extensions: &["clz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
