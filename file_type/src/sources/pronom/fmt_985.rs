use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_985: FileFormat = FileFormat {
    id: 1_790,
    puid: "fmt/985",
    name: "Valve Texture Format",
    extensions: &["vtf"],
    media_types: &["image/vnd.valve.source.texture"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x46, 0x54, 0x56])],
                },
            }],
        },
        InternalSignature {
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
