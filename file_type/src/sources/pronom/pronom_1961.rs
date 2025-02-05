use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1961: FileFormat = FileFormat {
    id: 1_961,
    source_type: SourceType::Pronom,
    name: "Lightwright Show File",
    extensions: &["lw1", "lw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x57, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
