use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2438: FileFormat = FileFormat {
    id: 2_438,
    source_type: SourceType::Pronom,
    name: "WRAptor Compressed File",
    extensions: &["wra", "wr3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x42, 0x4C, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
