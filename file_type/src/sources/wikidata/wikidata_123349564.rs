use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123349564: FileFormat = FileFormat {
    id: 123_349_564,
    source_type: SourceType::Wikidata,
    name: "Clooz database file",
    extensions: &["clz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
