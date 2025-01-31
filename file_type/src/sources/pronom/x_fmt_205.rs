use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_205: FileFormat = FileFormat {
    id: 285,
    puid: "x-fmt/205",
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x50]),
                    Token::WildcardCount(120),
                    Token::Literal(&[0x7D, 0x00, 0x1D]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 342,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 378,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
