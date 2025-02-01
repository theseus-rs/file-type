use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1478: FileFormat = FileFormat {
    id: 2_301,
    puid: "fmt/1478",
    name: "Unisig",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDC, 0xDC, 0x0D, 0x0A, 0x1A, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
