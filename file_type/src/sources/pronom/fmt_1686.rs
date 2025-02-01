use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1686: FileFormat = FileFormat {
    id: 2_522,
    puid: "fmt/1686",
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
            id: 516,
            relationship_type: RelationshipType::EquivalentTo,
        },
        RelatedFormat {
            id: 2_523,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 515,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
