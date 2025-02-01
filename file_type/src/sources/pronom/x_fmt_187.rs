use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_187: FileFormat = FileFormat {
    id: 260,
    puid: "x-fmt/187",
    name: "Painter RIFF Image File",
    extensions: &["rif"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x02, 0x00, 0x00]),
                        Token::WildcardCount(36),
                        Token::Literal(&[0x3F, 0xE6, 0x66, 0x66]),
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
                        Token::Literal(&[0x00, 0x02, 0x20, 0x00]),
                        Token::WildcardCount(36),
                        Token::Literal(&[0x3F, 0xE6, 0x66, 0x66]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
