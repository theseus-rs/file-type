use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123002780: FileFormat = FileFormat {
    id: 123_002_780,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics 1.1",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
