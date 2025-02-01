use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_688: FileFormat = FileFormat {
    id: 1_487,
    puid: "fmt/688",
    name: "Executable and Linkable Format",
    extensions: &["elf", "o"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46, 0x01, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
