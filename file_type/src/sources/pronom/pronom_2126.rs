use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2126: FileFormat = FileFormat {
    id: 2_126,
    source_type: SourceType::Pronom,
    name: "LocoScript PC",
    extensions: &[],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
