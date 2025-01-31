use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1155: FileFormat = FileFormat {
    id: 1_965,
    puid: "fmt/1155",
    name: "Lightwright Show File",
    extensions: &["lw5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4D, 0x43, 0x4B])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4C, 0x45, 0x41, 0x52, 0x20, 0x54, 0x45, 0x58, 0x54, 0x20, 0x42,
                        0x4C, 0x4F, 0x43, 0x4B, 0x0D, 0x0A, 0x4C, 0x57, 0x46, 0x4F, 0x52, 0x4D,
                        0x41, 0x54, 0x20, 0x56, 0x35,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
