use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1150: FileFormat = FileFormat {
    id: 1_960,
    puid: "fmt/1150",
    name: "4X Movie File",
    extensions: &["4xm", "4xa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x34, 0x58, 0x4D, 0x56, 0x4C, 0x49, 0x53, 0x54]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x48, 0x45, 0x41, 0x44]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_741,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
