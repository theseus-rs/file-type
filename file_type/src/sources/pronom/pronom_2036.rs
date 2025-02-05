use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2036: FileFormat = FileFormat {
    id: 2_036,
    source_type: SourceType::Pronom,
    name: "Sparky",
    extensions: &["ucsf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x55, 0x43, 0x53, 0x46, 0x20, 0x4E, 0x4D, 0x52, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x02]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
