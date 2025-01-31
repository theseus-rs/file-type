use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_173: FileFormat = FileFormat {
    id: 246,
    puid: "x-fmt/173",
    name: "PageMaker PC Document",
    extensions: &["pm5", "pt5"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(6),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x99]),
                    Token::WildcardCount(100),
                    Token::Literal(&[0x4D, 0x50, 0x00, 0x05]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_523,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 247,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 516,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
