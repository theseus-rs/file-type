use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1134: FileFormat = FileFormat {
    id: 1_134,
    source_type: SourceType::Pronom,
    name: "Microsoft Animated Cursor Format",
    extensions: &["ani"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x41, 0x43, 0x4F, 0x4E]),
                    Token::WildcardCountRange(0, 4_294_967_295),
                    Token::Literal(&[
                        0x61, 0x6E, 0x69, 0x68, 0x24, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                    ]),
                    Token::NotLiteral(&[0x00]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x4C, 0x49, 0x53, 0x54]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x66, 0x72, 0x61, 0x6D, 0x69, 0x63, 0x6F, 0x6E]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_741,
    }],
};
