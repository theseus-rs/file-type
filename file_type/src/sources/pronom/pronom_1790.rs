use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1790: FileFormat = FileFormat {
    id: 1_790,
    source_type: SourceType::Pronom,
    name: "Valve Texture Format",
    extensions: &["vtf"],
    media_types: &["image/vnd.valve.source.texture"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x46, 0x54, 0x56])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x54, 0x46, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
