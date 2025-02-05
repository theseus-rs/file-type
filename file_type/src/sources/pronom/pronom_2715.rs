use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2715: FileFormat = FileFormat {
    id: 2_715,
    source_type: SourceType::Pronom,
    name: "Quicken 3 Database File",
    extensions: &["qst"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x57, 0x20, 0x56, 0x65, 0x72, 0x2E, 0x20, 0x33, 0x2E, 0x30, 0x30, 0x2E,
                    0x30, 0x30, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
