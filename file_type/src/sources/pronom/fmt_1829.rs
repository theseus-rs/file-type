use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1829: FileFormat = FileFormat {
    id: 2_681,
    puid: "fmt/1829",
    name: "PPTX Strict OOXML Presentation",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 941,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
