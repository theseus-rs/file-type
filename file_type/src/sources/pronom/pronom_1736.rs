use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1736: FileFormat = FileFormat {
    id: 1_736,
    source_type: SourceType::Pronom,
    name: "Mathcad Document",
    extensions: &["mcd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x4D, 0x43, 0x41, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
