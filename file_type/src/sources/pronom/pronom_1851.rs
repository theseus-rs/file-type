use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1851: FileFormat = FileFormat {
    id: 1_851,
    source_type: SourceType::Pronom,
    name: "Draco File Format",
    extensions: &["drc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x52, 0x41, 0x43, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
