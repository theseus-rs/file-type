use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2382: FileFormat = FileFormat {
    id: 2_382,
    source_type: SourceType::Pronom,
    name: "Cyber Paint Sequence",
    extensions: &["seq"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xDB, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
