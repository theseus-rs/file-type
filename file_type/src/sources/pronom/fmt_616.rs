use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_616: FileFormat = FileFormat {
    id: 1_412,
    puid: "fmt/616",
    name: "Web Open Font Format",
    extensions: &["woff"],
    media_types: &["font/woff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x4F, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 869,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
