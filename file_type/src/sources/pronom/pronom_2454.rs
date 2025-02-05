use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2454: FileFormat = FileFormat {
    id: 2_454,
    source_type: SourceType::Pronom,
    name: "Z Print Build File",
    extensions: &["zbd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x5A, 0x43, 0x6F, 0x72, 0x70, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D,
                    0x20, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x56, 0x65, 0x72,
                    0x73, 0x69, 0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
