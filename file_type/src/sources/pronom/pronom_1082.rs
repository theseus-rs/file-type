use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1082: FileFormat = FileFormat {
    id: 1_082,
    source_type: SourceType::Pronom,
    name: "MJ2 (Motion JPEG 2000)",
    extensions: &["mj2", "mjp2"],
    media_types: &["video/mj2"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x66, 0x74, 0x79, 0x70, 0x6D, 0x6A, 0x70, 0x32]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
