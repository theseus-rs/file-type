use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1630: FileFormat = FileFormat {
    id: 1_630,
    source_type: SourceType::Pronom,
    name: "3MF 3D Manufacturing Format",
    extensions: &["3mf"],
    media_types: &["application/vnd.ms-3mfdocument"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_456,
    }],
};
