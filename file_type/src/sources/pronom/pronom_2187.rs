use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2187: FileFormat = FileFormat {
    id: 2_187,
    source_type: SourceType::Pronom,
    name: "Error File",
    extensions: &["err"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
