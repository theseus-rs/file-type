use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_700: FileFormat = FileFormat {
    id: 1_499,
    puid: "fmt/700",
    name: "Industry Foundation Classes",
    extensions: &["ifc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x49, 0x53, 0x4F, 0x2D, 0x31, 0x30, 0x33, 0x30, 0x33, 0x2D, 0x32, 0x31,
                    ]),
                    Token::WildcardCountRange(0, 4_096),
                    Token::Literal(&[
                        0x46, 0x49, 0x4C, 0x45, 0x5F, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x41,
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x28]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x28, 0x27, 0x49, 0x46, 0x43, 0x34]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_497,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_498,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
