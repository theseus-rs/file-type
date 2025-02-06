use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1517: FileFormat = FileFormat {
    id: 1_517,
    source_type: SourceType::Pronom,
    name: "Scream Tracker Module",
    extensions: &["s3m"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(28),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x1A, 0x10, 0x00, 0x00]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x53, 0x43, 0x52, 0x4D]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
