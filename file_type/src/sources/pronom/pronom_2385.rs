use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2385: FileFormat = FileFormat {
    id: 2_385,
    source_type: SourceType::Pronom,
    name: "Sample Vision Audio File Format",
    extensions: &["smp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4F, 0x55, 0x4E, 0x44, 0x20, 0x53, 0x41, 0x4D, 0x50, 0x4C, 0x45, 0x20,
                    0x44, 0x41, 0x54, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
