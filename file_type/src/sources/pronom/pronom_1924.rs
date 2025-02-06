use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1924: FileFormat = FileFormat {
    id: 1_924,
    source_type: SourceType::Pronom,
    name: "Python Compiled File",
    extensions: &["pyc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x9E, 0x0C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
