use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_237: FileFormat = FileFormat {
    id: 343,
    puid: "x-fmt/237",
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws6"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
            id: 379,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 342,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
