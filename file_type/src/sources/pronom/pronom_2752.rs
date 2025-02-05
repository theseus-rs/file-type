use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2752: FileFormat = FileFormat {
    id: 2_752,
    source_type: SourceType::Pronom,
    name: "Nokia Picture Message",
    extensions: &["npm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
