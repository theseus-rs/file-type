use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123002751: FileFormat = FileFormat {
    id: 123_002_751,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics 1.0",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    signatures: &[],
    related_formats: &[],
};
