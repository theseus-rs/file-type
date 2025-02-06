use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1256: FileFormat = FileFormat {
    id: 1_256,
    source_type: SourceType::Pronom,
    name: "MS DOS Compression Format (KWAJ Variant)",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
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
