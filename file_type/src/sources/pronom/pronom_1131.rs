use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1131: FileFormat = FileFormat {
    id: 1_131,
    source_type: SourceType::Pronom,
    name: "Microsoft Visual FoxPro database container (memo files)",
    extensions: &["dct"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::HasPriorityOver,
        id: 506,
    }],
};
