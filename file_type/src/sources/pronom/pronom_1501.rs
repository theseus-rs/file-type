use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1501: FileFormat = FileFormat {
    id: 1_501,
    source_type: SourceType::Pronom,
    name: "Universal 3D File Format",
    extensions: &["u3d"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x33, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
