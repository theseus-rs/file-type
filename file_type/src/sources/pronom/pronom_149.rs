use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_149: FileFormat = FileFormat {
    id: 149,
    source_type: SourceType::Pronom,
    name: "Harvard Graphics Show",
    extensions: &["sh3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x47, 0x42, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
