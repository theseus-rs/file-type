use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2665: FileFormat = FileFormat {
    id: 2_665,
    source_type: SourceType::Pronom,
    name: "Adobe Color Book for Windows",
    extensions: &["acb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x42, 0x43, 0x42, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
