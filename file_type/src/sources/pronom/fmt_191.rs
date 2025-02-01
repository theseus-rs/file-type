use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_191: FileFormat = FileFormat {
    id: 916,
    puid: "fmt/191",
    name: "Sony ARW RAW Image File",
    extensions: &["arw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                    Token::WildcardCountRange(0, 512),
                    Token::Literal(&[0x53, 0x4F, 0x4E, 0x59]),
                    Token::WildcardCountRange(0, 2_048),
                    Token::Literal(&[0x00, 0xB0, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_099,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
