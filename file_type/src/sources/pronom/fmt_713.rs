use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_713: FileFormat = FileFormat {
    id: 1_512,
    puid: "fmt/713",
    name: "RF64 Multichannel Broadcast Wave format",
    extensions: &["wav", "rf64"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x46, 0x36, 0x34]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x64, 0x73, 0x36, 0x34]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x62, 0x65, 0x78, 0x74]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_511,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
