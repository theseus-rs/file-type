use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123436632: FileFormat = FileFormat {
    id: 123_436_632,
    source_type: SourceType::Wikidata,
    name: "Construction File",
    extensions: &["cf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
