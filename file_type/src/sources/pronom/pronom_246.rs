use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_246: FileFormat = FileFormat {
    id: 246,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::EquivalentTo,
            id: 2_523,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 247,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 516,
        },
    ],
};
