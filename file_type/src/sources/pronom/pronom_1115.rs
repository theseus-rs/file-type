use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1115: FileFormat = FileFormat {
    id: 1_115,
    source_type: SourceType::Pronom,
    name: "ASPRS Lidar Data Exchange Format",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4C, 0x41, 0x53, 0x46]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x01, 0x00]),
                    Token::WildcardCount(78),
                    Token::Range(&[0x00], &[0x99]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_116,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_117,
        },
    ],
};
