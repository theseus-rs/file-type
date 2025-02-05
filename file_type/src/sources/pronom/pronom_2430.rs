use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2430: FileFormat = FileFormat {
    id: 2_430,
    source_type: SourceType::Pronom,
    name: "TUNDRA",
    extensions: &["tnd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x54, 0x55, 0x4E, 0x44, 0x52, 0x41, 0x32, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
