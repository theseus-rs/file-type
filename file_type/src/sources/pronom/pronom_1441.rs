use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1441: FileFormat = FileFormat {
    id: 1_441,
    source_type: SourceType::Pronom,
    name: "Fujifilm RAW Image Format",
    extensions: &["raf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x4A, 0x49, 0x46, 0x49, 0x4C, 0x4D, 0x43, 0x43, 0x44, 0x2D, 0x52,
                    0x41, 0x57, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
