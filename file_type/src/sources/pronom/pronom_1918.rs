use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1918: FileFormat = FileFormat {
    id: 1_918,
    source_type: SourceType::Pronom,
    name: "Python Compiled File",
    extensions: &["pyc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0xF2, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
