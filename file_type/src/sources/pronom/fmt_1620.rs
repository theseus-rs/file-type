use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1620: FileFormat = FileFormat {
    id: 2_447,
    puid: "fmt/1620",
    name: "Aero Studio Song",
    extensions: &["aero"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x45, 0x52, 0x4F, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40])],
                },
            },
        ],
    }],
    related_formats: &[],
};
