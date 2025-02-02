use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2522: FileFormat = FileFormat {
    id: 2_522,
    source_type: SourceType::Pronom,
    name: "PageMaker Mac Document",
    extensions: &[],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(6),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x99, 0xFF]),
                    Token::WildcardCount(100),
                    Token::Literal(&[0x50, 0x4D, 0x04, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::EquivalentTo,
            id: 516,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_523,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 515,
        },
    ],
};
