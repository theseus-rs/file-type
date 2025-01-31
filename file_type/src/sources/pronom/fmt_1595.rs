use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1595: FileFormat = FileFormat {
    id: 2_422,
    puid: "fmt/1595",
    name: "Canon Raw",
    extensions: &["cr3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x63, 0x72, 0x78, 0x20]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x6D, 0x6F, 0x6F, 0x76]),
                    Token::WildcardCount(32),
                    Token::Literal(&[0x43]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_421,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 924,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
