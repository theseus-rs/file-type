use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2377: FileFormat = FileFormat {
    id: 2_377,
    source_type: SourceType::Pronom,
    name: "Surprise! Adlib Tracker v2.0",
    extensions: &["sa2"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
