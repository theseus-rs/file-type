use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1587: FileFormat = FileFormat {
    id: 2_412,
    puid: "fmt/1587",
    name: "COKE Format (Atari Falcon)",
    extensions: &["tg1"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4B, 0x45, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
