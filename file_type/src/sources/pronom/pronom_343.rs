use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_343: FileFormat = FileFormat {
    id: 343,
    source_type: SourceType::Pronom,
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws6"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x60]),
                    Token::WildcardCount(120),
                    Token::Literal(&[0x7D, 0x00, 0x1D]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 379,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 342,
        },
    ],
};
