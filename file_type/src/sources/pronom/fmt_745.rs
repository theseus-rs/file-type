use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_745: FileFormat = FileFormat {
    id: 1_544,
    puid: "fmt/745",
    name: "ClarisWorks/AppleWorks Spreadsheet",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x05]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                    Token::WildcardCount(260),
                    Token::Literal(&[0x02]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_549,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_539,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
