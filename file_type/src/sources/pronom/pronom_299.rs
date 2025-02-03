use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_299: FileFormat = FileFormat {
    id: 299,
    source_type: SourceType::Pronom,
    name: "Lotus 1-2-3 Worksheet",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
