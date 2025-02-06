use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2614: FileFormat = FileFormat {
    id: 2_614,
    source_type: SourceType::Pronom,
    name: "Sony SLV File",
    extensions: &["slv"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x73, 0x6C, 0x76, 0x67]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x66, 0x69, 0x6C, 0x6C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
