use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1500: FileFormat = FileFormat {
    id: 1_500,
    source_type: SourceType::Pronom,
    name: "Processing Development Environment",
    extensions: &["pde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
