use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58630708: FileFormat = FileFormat {
    id: 58_630_708,
    source_type: SourceType::Wikidata,
    name: "Scalable Vector Graphics Tiny",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    signatures: &[],
    related_formats: &[],
};
