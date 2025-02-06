use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1954: FileFormat = FileFormat {
    id: 1_954,
    source_type: SourceType::Pronom,
    name: "CompuServe WinCIM Message Format",
    extensions: &["plx", "msg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x49, 0x53, 0x30, 0x30, 0x30, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
