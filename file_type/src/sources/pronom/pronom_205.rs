use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_205: FileFormat = FileFormat {
    id: 205,
    source_type: SourceType::Pronom,
    name: "Corel Photo-Paint Image",
    extensions: &["cpt"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x54, 0x37, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x54, 0x38, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x54, 0x39, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
