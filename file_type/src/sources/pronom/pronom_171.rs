use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_171: FileFormat = FileFormat {
    id: 171,
    source_type: SourceType::Pronom,
    name: "Windows Metafile Image",
    extensions: &["wmf"],
    media_types: &["image/wmf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xD7, 0xCD, 0xC6, 0x9A, 0x00, 0x00]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x01, 0x00, 0x09, 0x00]),
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
                        Token::Literal(&[0x01, 0x00, 0x09, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x03])]]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
