use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1870: FileFormat = FileFormat {
    id: 2_724,
    puid: "fmt/1870",
    name: "Yamaha PSR Disk Manager File",
    extensions: &["mng"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4E, 0x47])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A])],
                },
            },
        ],
    }],
    related_formats: &[],
};
