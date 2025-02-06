use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2406: FileFormat = FileFormat {
    id: 2_406,
    source_type: SourceType::Pronom,
    name: "Envision Publisher Font Files",
    extensions: &["svf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x83, 0xF9, 0xF8, 0xF8, 0xF9, 0xF9, 0xF9, 0x11, 0xFA, 0xF8, 0xF9, 0xF8,
                    0xF9, 0xD9, 0xF9, 0x19, 0xF9,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
