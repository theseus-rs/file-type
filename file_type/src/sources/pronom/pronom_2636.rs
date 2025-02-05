use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2636: FileFormat = FileFormat {
    id: 2_636,
    source_type: SourceType::Pronom,
    name: "Funpaint Image File",
    extensions: &["fun", "fp2", "vic"],
    media_types: &[],
    signatures: &[Signature {
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
