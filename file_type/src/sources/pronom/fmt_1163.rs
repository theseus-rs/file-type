use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1163: FileFormat = FileFormat {
    id: 1_973,
    puid: "fmt/1163",
    name: "Folio Definition File",
    extensions: &["def"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x43, 0x4D, 0x3E])],
                },
            },
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x6F, 0x6C, 0x69, 0x6F, 0x2C, 0x44, 0x45, 0x46,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
