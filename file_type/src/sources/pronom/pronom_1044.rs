use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1044: FileFormat = FileFormat {
    id: 1_044,
    source_type: SourceType::Pronom,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x43, 0x48, 0x49, 0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x20, 0x34, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
