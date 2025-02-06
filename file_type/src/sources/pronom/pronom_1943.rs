use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1943: FileFormat = FileFormat {
    id: 1_943,
    source_type: SourceType::Pronom,
    name: "Farbfeld Image Format",
    extensions: &["ff"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x61, 0x72, 0x62, 0x66, 0x65, 0x6C, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
