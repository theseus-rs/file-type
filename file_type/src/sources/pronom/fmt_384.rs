use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_384: FileFormat = FileFormat {
    id: 1_131,
    puid: "fmt/384",
    name: "Microsoft Visual FoxPro database container (memo files)",
    extensions: &["dct"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x40]),
                    Token::WildcardCount(504),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x0B, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 506,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
