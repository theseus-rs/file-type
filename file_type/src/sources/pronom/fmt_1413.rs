use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1413: FileFormat = FileFormat {
    id: 2_231,
    puid: "fmt/1413",
    name: "Corel Gallery Clipart",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x43, 0x6F, 0x72, 0x65, 0x6C, 0x42, 0x4D, 0x46, 0x0A, 0x0D, 0x43, 0x6F,
                    0x72, 0x65, 0x6C, 0x20, 0x43, 0x6F, 0x72, 0x70, 0x6F, 0x72, 0x61, 0x74, 0x69,
                    0x6F, 0x6E, 0x0A, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
