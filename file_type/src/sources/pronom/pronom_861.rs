use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_861: FileFormat = FileFormat {
    id: 861,
    source_type: SourceType::Pronom,
    name: "Revit External Group",
    extensions: &["rvg"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubtypeOf,
        id: 767,
    }],
};
