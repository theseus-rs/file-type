use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1966: FileFormat = FileFormat {
    id: 2_831,
    puid: "fmt/1966",
    name: "Final Writer Document",
    extensions: &["fw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x53, 0x57, 0x52, 0x54]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 221,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
