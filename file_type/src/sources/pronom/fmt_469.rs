use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_469: FileFormat = FileFormat {
    id: 1_256,
    puid: "fmt/469",
    name: "MS DOS Compression Format (KWAJ Variant)",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x57, 0x41, 0x4A, 0x88, 0xF0, 0x27, 0xD1,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
