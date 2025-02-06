use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_387: FileFormat = FileFormat {
    id: 387,
    source_type: SourceType::Pronom,
    name: "BZIP Compressed Archive",
    extensions: &["bz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x5A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
