use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2678: FileFormat = FileFormat {
    id: 2_678,
    source_type: SourceType::Pronom,
    name: "DOCX Strict OOXML Document",
    extensions: &["docx"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 1_160,
    }],
};
