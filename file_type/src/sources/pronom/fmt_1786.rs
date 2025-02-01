use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1786: FileFormat = FileFormat {
    id: 2_636,
    puid: "fmt/1786",
    name: "Funpaint Image File",
    extensions: &["fun", "fp2", "vic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xF0, 0x3F, 0x46, 0x55, 0x4E, 0x50, 0x41, 0x49, 0x4E, 0x54, 0x20, 0x28, 0x4D,
                    0x54, 0x29, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
