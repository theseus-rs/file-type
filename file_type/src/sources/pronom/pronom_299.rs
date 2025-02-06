use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_299: FileFormat = FileFormat {
    id: 299,
    source_type: SourceType::Pronom,
    name: "Lotus 1-2-3 Worksheet",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
