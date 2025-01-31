use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_829: FileFormat = FileFormat {
    id: 1_630,
    puid: "fmt/829",
    name: "3MF 3D Manufacturing Format",
    extensions: &["3mf"],
    media_types: &["application/vnd.ms-3mfdocument"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_456,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
