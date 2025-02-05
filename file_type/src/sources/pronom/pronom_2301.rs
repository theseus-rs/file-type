use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2301: FileFormat = FileFormat {
    id: 2_301,
    source_type: SourceType::Pronom,
    name: "Unisig",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDC, 0xDC, 0x0D, 0x0A, 0x1A, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
