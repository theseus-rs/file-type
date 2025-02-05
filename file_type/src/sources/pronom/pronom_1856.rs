use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1856: FileFormat = FileFormat {
    id: 1_856,
    source_type: SourceType::Pronom,
    name: "Windows Journal Format",
    extensions: &["jnt", "jtp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4E, 0x42, 0x2A, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x50, 0x01, 0x00, 0x10, 0x02, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
