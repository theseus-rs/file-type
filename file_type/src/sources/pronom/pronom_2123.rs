use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2123: FileFormat = FileFormat {
    id: 2_123,
    source_type: SourceType::Pronom,
    name: "LocoScript Document",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4F, 0x59, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
