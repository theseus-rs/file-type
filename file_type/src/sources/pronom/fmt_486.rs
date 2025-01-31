use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_486: FileFormat = FileFormat {
    id: 1_273,
    puid: "fmt/486",
    name: "Macromedia (Adobe) Director Compressed Resource file",
    extensions: &["dcr"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x58]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x46, 0x47, 0x44, 0x4D]),
                    ],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x58, 0x46, 0x49, 0x52]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x4D, 0x44, 0x47, 0x46]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
