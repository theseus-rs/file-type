use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2027: FileFormat = FileFormat {
    id: 2_027,
    source_type: SourceType::Pronom,
    name: "Leonardo Image Format",
    extensions: &["leo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x65, 0x6F, 0x20, 0x7A, 0x6F, 0x6E, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D,
                    0x61, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
