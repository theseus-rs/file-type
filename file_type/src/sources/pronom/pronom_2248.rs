use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2248: FileFormat = FileFormat {
    id: 2_248,
    source_type: SourceType::Pronom,
    name: "Minitab Worksheet",
    extensions: &["mtw"],
    media_types: &[],
    signatures: &[Signature {
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
