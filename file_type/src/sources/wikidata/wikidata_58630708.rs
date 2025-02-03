use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58630708: FileFormat = FileFormat {
    id: 58_630_708,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics Tiny",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
