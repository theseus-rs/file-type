use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_352: FileFormat = FileFormat {
    id: 516,
    puid: "x-fmt/352",
    name: "PageMaker PC Document",
    extensions: &["pm4", "pt4"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(6),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x99]),
                    Token::WildcardCount(100),
                    Token::Literal(&[0x4D, 0x50, 0x00, 0x04]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_522,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 246,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 515,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
