use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1318: FileFormat = FileFormat {
    id: 2_136,
    puid: "fmt/1318",
    name: "QuarkXPress Document",
    extensions: &["qxd", "qxt", "qwd"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x3F]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x58, 0x50, 0x52]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x3F]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 255,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
