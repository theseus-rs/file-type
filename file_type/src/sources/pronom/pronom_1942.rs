use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1942: FileFormat = FileFormat {
    id: 1_942,
    source_type: SourceType::Pronom,
    name: "Netscape Bookmark File Format",
    extensions: &["htm", "html"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4E, 0x45, 0x54,
                    0x53, 0x43, 0x41, 0x50, 0x45, 0x2D, 0x42, 0x6F, 0x6F, 0x6B, 0x6D, 0x61, 0x72,
                    0x6B, 0x2D, 0x66, 0x69, 0x6C, 0x65, 0x2D, 0x31, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
