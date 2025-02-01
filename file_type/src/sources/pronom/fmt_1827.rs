use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1827: FileFormat = FileFormat {
    id: 2_678,
    puid: "fmt/1827",
    name: "DOCX Strict OOXML Document",
    extensions: &["docx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_160,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
