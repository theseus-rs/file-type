use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1500: FileFormat = FileFormat {
    id: 1_500,
    source_type: SourceType::Pronom,
    name: "Processing Development Environment",
    extensions: &["pde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
