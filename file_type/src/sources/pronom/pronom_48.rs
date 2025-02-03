use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_48: FileFormat = FileFormat {
    id: 48,
    source_type: SourceType::Pronom,
    name: "3D Studio",
    extensions: &["3ds"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x02, 0x00, 0x0A, 0x00, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x3D, 0x3D]),
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
                        Token::Literal(&[0x4D, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x3D, 0x3D]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
