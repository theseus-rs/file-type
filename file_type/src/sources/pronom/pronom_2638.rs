use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2638: FileFormat = FileFormat {
    id: 2_638,
    source_type: SourceType::Pronom,
    name: "Gunpaint Image File",
    extensions: &["gun"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x40]),
                    Token::WildcardCount(1_000),
                    Token::Literal(&[0x47, 0x55, 0x4E, 0x50, 0x41, 0x49, 0x4E, 0x54]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
