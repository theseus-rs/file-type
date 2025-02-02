use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_379: FileFormat = FileFormat {
    id: 379,
    source_type: SourceType::Pronom,
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x70]),
                    Token::WildcardCount(120),
                    Token::Literal(&[0x7D, 0x00, 0x1D]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_686,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 343,
        },
    ],
};
