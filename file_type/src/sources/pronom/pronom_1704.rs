use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1704: FileFormat = FileFormat {
    id: 1_704,
    source_type: SourceType::Pronom,
    name: "Windows Portable Executable",
    extensions: &["exe", "dll", "sys"],
    media_types: &["application/vnd.microsoft.portable-executable"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x5A]),
                    Token::WildcardCountRange(126, 128_500),
                    Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                    Token::WildcardCount(20),
                    Token::Literal(&[0x0B, 0x01]),
                    Token::WildcardCount(66),
                    Token::Range(&[0x00, 0x00], &[0x10, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_645,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 774,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 775,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 776,
        },
    ],
};
