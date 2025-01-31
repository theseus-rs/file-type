use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1430: FileFormat = FileFormat {
    id: 2_248,
    puid: "fmt/1430",
    name: "Minitab Worksheet",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x4F, 0x52, 0x4B, 0x53, 0x48, 0x45, 0x45, 0x54, 0x20, 0x53, 0x54, 0x4F,
                    0x52, 0x45, 0x44, 0x20, 0x42, 0x59, 0x20, 0x4D, 0x49, 0x4E, 0x49, 0x54, 0x41,
                    0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
