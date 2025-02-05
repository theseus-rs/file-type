use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2556: FileFormat = FileFormat {
    id: 2_556,
    source_type: SourceType::Pronom,
    name: "Portable Compiled Format",
    extensions: &["pcf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x66, 0x63, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
